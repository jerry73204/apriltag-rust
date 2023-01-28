//! Image types for AprilTag detection.
//!
//! The [Image] type stores the image in single channel byte buffer.
//! It can be created by
//! [zeros_with_stride](Image::zeros_with_stride),
//! [zeros_with_alignment](Image::zeros_with_alignment) or converted
//! from third-party types using extension crates.

use crate::Error;
use apriltag_sys as sys;
use std::{
    ffi::{c_int, c_uint, CString},
    iter,
    mem::ManuallyDrop,
    ops::{Index, IndexMut},
    ptr::NonNull,
    slice,
};

pub const DEFAULT_ALIGNMENT_U8: usize = 96;

/// The single-channel image with pixels in bytes.
#[derive(Debug)]
#[repr(transparent)]
pub struct Image {
    pub(crate) ptr: NonNull<sys::image_u8_t>,
}

impl Image {
    /// Give width and height and create an uninitialized image.
    ///
    /// # Safety
    ///
    /// After the image is returned, the caller must explicitly
    /// initialize the image buffer.
    pub unsafe fn new_uinit(width: usize, height: usize) -> Result<Self, Error> {
        let ptr = sys::image_u8_create(width as c_uint, height as c_uint);
        let ptr = NonNull::new(ptr).ok_or_else(|| Error::CreateImageError {
            reason: "image_u8_create() failed".to_string(),
        })?;
        Ok(Self { ptr })
    }

    /// Give width, height and stride and create an uninitialized image.
    ///
    /// # Safety
    ///
    /// After the image is returned, the caller must explicitly
    /// initialize the image buffer.
    pub unsafe fn new_uinit_with_stride(
        width: usize,
        height: usize,
        stride: usize,
    ) -> Result<Self, Error> {
        let ptr = sys::image_u8_create_stride(width as c_uint, height as c_uint, stride as c_uint);
        let ptr = NonNull::new(ptr).ok_or_else(|| Error::CreateImageError {
            reason: "image_u8_create() failed".to_string(),
        })?;
        Ok(Self { ptr })
    }

    /// Create an image from a PNM file.
    pub fn from_pnm_file(path: &str) -> Result<Self, Error> {
        let cstr = CString::new(path).map_err(|_| Error::CreateImageError {
            reason: format!("the path '{path}' contains null byte(s)"),
        })?;
        let ptr = unsafe { sys::image_u8_create_from_pnm(cstr.as_ptr()) };
        let ptr = NonNull::new(ptr).ok_or_else(|| Error::CreateImageError {
            reason: format!("failed to load '{path}' using image_u8_create_from_pnm()"),
        })?;
        Ok(Self { ptr })
    }

    /// Create an image from a PNM file with a specified alignment.
    pub fn from_pnm_file_with_alignment(path: &str, alignment: usize) -> Result<Self, Error> {
        let cstr = CString::new(path).map_err(|_| Error::CreateImageError {
            reason: format!("the path '{path}' contains null byte(s)"),
        })?;
        let ptr =
            unsafe { sys::image_u8_create_from_pnm_alignment(cstr.as_ptr(), alignment as c_int) };
        let ptr = NonNull::new(ptr).ok_or_else(|| Error::CreateImageError {
            reason: format!("failed to load '{path}' using image_u8_create_from_pnm()"),
        })?;
        Ok(Self { ptr })
    }

    /// Create a zerod image.
    ///
    /// The `stride` must be more than or equal to `width`. Otherwise it returns `None`.
    pub fn zeros_with_stride(width: usize, height: usize, stride: usize) -> Result<Self, Error> {
        if width > stride {
            return Err(Error::CreateImageError {
                reason: format!("width ({width}) must be less than or equal to stride ({stride})"),
            });
        }

        let ptr = unsafe {
            sys::image_u8_create_stride(width as c_uint, height as c_uint, stride as c_uint)
        };
        let ptr = NonNull::new(ptr).ok_or_else(|| Error::CreateImageError {
            reason: "image_u8_create_stride() failed".to_string(),
        })?;

        Ok(Self { ptr })
    }

    /// Create a zerod image.
    ///
    /// The `alignment` must be non-zero. Otherwise it returns `None`.
    pub fn zeros_with_alignment(
        width: usize,
        height: usize,
        alignment: usize,
    ) -> Result<Self, Error> {
        if alignment == 0 {
            return Err(Error::CreateImageError {
                reason: format!("alignment ({alignment}) must positive"),
            });
        }

        let ptr = unsafe {
            sys::image_u8_create_alignment(width as c_uint, height as c_uint, alignment as c_uint)
        };
        let ptr = NonNull::new(ptr).ok_or_else(|| Error::CreateImageError {
            reason: "image_u8_create_alignment() failed".to_string(),
        })?;

        Ok(Self { ptr })
    }

    /// Create an iterator traversing pixels in row-major order.
    pub fn samples_iter(&self) -> impl Iterator<Item = u8> + '_ {
        let height = self.height();
        let width = self.width();
        let stride = self.stride();
        let buffer = self.as_slice();

        let row_offsets =
            iter::successors(Some(0), move |&offset| Some(offset + stride)).take(height);
        let pixel_offsets =
            row_offsets.flat_map(move |row_offset| row_offset..(row_offset + width));
        pixel_offsets.map(move |offset| buffer[offset])
    }

    /// Create an iterator that traverses pixels with pixel positions
    /// in row-major order.
    ///
    /// The iterator item is in (x, y, pixel_value) format.
    pub fn indexed_samples_iter(&self) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
        let height = self.height();
        let width = self.width();
        let stride = self.stride();
        let buffer = self.as_slice();

        let row_offsets = iter::successors(Some(0), move |&offset| Some(offset + stride))
            .take(height)
            .enumerate();
        let pixel_offsets = row_offsets.flat_map(move |(row, row_offset)| {
            let offsets = row_offset..(row_offset + width);
            offsets
                .enumerate()
                .map(move |(col, offset)| (row, col, offset))
        });
        pixel_offsets.map(move |(row, col, offset)| (col, row, buffer[offset]))
    }

    /// Gets the image width.
    pub fn width(&self) -> usize {
        unsafe { self.ptr.as_ref().width as usize }
    }

    /// Gets the image height.
    pub fn height(&self) -> usize {
        unsafe { self.ptr.as_ref().height as usize }
    }

    /// Gets the per-row stride in bytes.
    pub fn stride(&self) -> usize {
        unsafe { self.ptr.as_ref().stride as usize }
    }

    /// Get the pixel buffer with size height*stride.
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            let image = self.ptr.as_ref();
            let len = (image.height * image.stride) as usize;
            slice::from_raw_parts(image.buf, len)
        }
    }

    /// Get the mutable pixel buffer with size height*stride.
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe {
            let image = self.ptr.as_ref();
            let len = (image.height * image.stride) as usize;
            slice::from_raw_parts_mut(image.buf, len)
        }
    }

    /// Creates an instance from pointer.
    ///
    /// The pointer will be managed by the type. Do not run manual deallocation on the pointer.
    /// Panics if the pointer is null.
    ///
    /// # Safety
    /// The method is safe when the pointer was created by [image_u8_create_stride](sys::image_u8_create_stride) or [image_u8_create_alignment](sys::image_u8_create_alignment).
    pub unsafe fn from_raw(ptr: *mut sys::image_u8_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Returns the underlying pointer.
    pub fn into_raw(self) -> NonNull<sys::image_u8_t> {
        ManuallyDrop::new(self).ptr
    }
}

impl Index<(usize, usize)> for Image {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let stride = self.stride();
        &self.as_ref()[x + y * stride]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let stride = self.stride();
        &mut self.as_mut()[x + y * stride]
    }
}

impl AsRef<[u8]> for Image {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr.as_ref().buf, self.stride() * self.height()) }
    }
}

impl AsMut<[u8]> for Image {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ref().buf, self.stride() * self.height()) }
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        let cloned_ptr = unsafe { sys::image_u8_copy(self.ptr.as_ptr()) };
        Self {
            ptr: NonNull::new(cloned_ptr).unwrap(),
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            sys::image_u8_destroy(self.ptr.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_clone() {
        let width = 80;
        let height = 60;
        let from_image = diagonal_image(width, height);
        let to_image = from_image.clone();
        assert_eq!(from_image.width(), to_image.width());
        assert_eq!(from_image.height(), to_image.height());
        assert_eq!(from_image.stride(), to_image.stride());

        // invert color of to_image
        (0..height)
            .into_iter()
            .flat_map(|y| (0..width).into_iter().map(move |x| (x, y)))
            .for_each(|(x, y)| {
                assert_eq!(from_image[(x, y)], to_image[(x, y)]);
            });
    }

    fn diagonal_image(width: usize, height: usize) -> Image {
        let mut image = Image::zeros_with_alignment(width, height, DEFAULT_ALIGNMENT_U8).unwrap();
        (0..width.min(height)).into_iter().for_each(|index| {
            image[(index, index)] = 255;
        });
        image
    }
}
