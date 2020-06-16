use apriltag_sys as sys;
use std::{
    ops::{Deref, Index, IndexMut},
    os::raw::c_uint,
    ptr::NonNull,
    slice,
};

const DEFAULT_ALIGNMENT_U8: usize = 96;

#[derive(Debug)]
pub struct Image {
    pub(crate) ptr: NonNull<sys::image_u8_t>,
}

impl Image {
    pub fn zeros_stride(width: usize, height: usize, stride: usize) -> Option<Self> {
        if width > stride {
            return None;
        }

        let ptr = unsafe {
            sys::image_u8_create_stride(width as c_uint, height as c_uint, stride as c_uint)
        };

        Some(Self {
            ptr: NonNull::new(ptr).unwrap(),
        })
    }

    pub fn zeros_alignment(width: usize, height: usize, alignment: usize) -> Option<Self> {
        if alignment == 0 {
            return None;
        }

        let ptr = unsafe {
            sys::image_u8_create_alignment(width as c_uint, height as c_uint, alignment as c_uint)
        };

        Some(Self {
            ptr: NonNull::new(ptr).unwrap(),
        })
    }

    pub fn samples_iter<'a>(&'a self) -> SamplesIter<'a> {
        SamplesIter {
            image: self,
            width: self.width(),
            height: self.height(),
            index: (0, 0),
        }
    }

    pub fn width(&self) -> usize {
        unsafe { self.ptr.as_ref().width as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { self.ptr.as_ref().height as usize }
    }

    pub fn stride(&self) -> usize {
        unsafe { self.ptr.as_ref().stride as usize }
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

#[derive(Debug, Clone)]
pub struct SamplesIter<'a> {
    image: &'a Image,
    width: usize,
    height: usize,
    index: (usize, usize),
}

impl<'a> Iterator for SamplesIter<'a> {
    type Item = (usize, usize, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.index;

        let (next_x, next_y, value) = if x + 1 < self.width {
            (x + 1, y, self.image[(x, y)])
        } else if y + 1 < self.height {
            (0, y + 1, self.image[(x, y)])
        } else {
            return None;
        };

        self.index = (next_x, next_y);
        Some((x, y, value))
    }
}

#[cfg(feature = "nalgebra")]
mod nalgebra_conv {
    use super::*;
    use nalgebra::{
        base::{
            dimension::{Dim, Dynamic},
            storage::Storage,
        },
        Matrix, MatrixMN,
    };

    impl From<&Image> for MatrixMN<u8, Dynamic, Dynamic> {
        fn from(from: &Image) -> Self {
            let width = from.width();
            let height = from.height();
            Self::from_fn(height, width, |row, col| from[(col, row)])
        }
    }

    impl From<Image> for MatrixMN<u8, Dynamic, Dynamic> {
        fn from(from: Image) -> Self {
            Self::from(&from)
        }
    }

    impl<R, C, S> From<&Matrix<u8, R, C, S>> for Image
    where
        R: Dim,
        C: Dim,
        S: Storage<u8, R, C>,
    {
        fn from(from: &Matrix<u8, R, C, S>) -> Self {
            let width = from.ncols();
            let height = from.nrows();
            let mut to = Image::zeros_alignment(width, height, DEFAULT_ALIGNMENT_U8);

            from.row_iter()
                .enumerate()
                .flat_map(|(row_idx, row)| {
                    row.iter()
                        .enumerate()
                        .map(move |(col_idx, value)| (row_idx, col_idx, *value))
                        .collect::<Vec<_>>()
                })
                .for_each(|(row_idx, col_idx, value)| {
                    to[(col_idx, row_idx)] = value;
                });
            to
        }
    }

    impl<R, C, S> From<Matrix<u8, R, C, S>> for Image
    where
        R: Dim,
        C: Dim,
        S: Storage<u8, R, C>,
    {
        fn from(from: Matrix<u8, R, C, S>) -> Self {
            Self::from(&from)
        }
    }
}

#[cfg(feature = "image")]
mod image_conv {
    use super::*;
    use image::{
        flat::{FlatSamples, SampleLayout},
        ColorType, ImageBuffer, Luma, Pixel,
    };

    impl<Buffer> From<&FlatSamples<Buffer>> for Image
    where
        Buffer: AsRef<[u8]>,
    {
        fn from(from: &FlatSamples<Buffer>) -> Self {
            match from.color_hint {
                Some(ColorType::L8) => (),
                _ => panic!("color type {:?} is not supported", from.color_hint),
            }

            let SampleLayout { width, height, .. } = from.layout;
            let mut image =
                Self::zeros_alignment(width as usize, height as usize, DEFAULT_ALIGNMENT_U8)
                    .unwrap();
            let stride = image.stride();

            let sample_iter = (0..height)
                .flat_map(|y| (0..width).map(move |x| (x, y)))
                .map(|(x, y)| *from.get_sample(0, x, y).unwrap());
            let buffer_index_iter = (0..height)
                .flat_map(|y| (0..width).map(move |x| (x as usize, y as usize)))
                .map(|(x, y)| y * stride + x);

            buffer_index_iter
                .zip(sample_iter)
                .for_each(|(buffer_index, sample)| {
                    image.as_mut()[buffer_index] = sample;
                });

            image
        }
    }

    impl<Buffer> From<FlatSamples<Buffer>> for Image
    where
        Buffer: AsRef<[u8]>,
    {
        fn from(from: FlatSamples<Buffer>) -> Self {
            Image::from(&from)
        }
    }

    impl From<&Image> for FlatSamples<Vec<u8>> {
        fn from(from: &Image) -> Self {
            let width = from.width();
            let height = from.height();
            let stride = from.stride();

            let mut samples = vec![];
            samples.extend_from_slice(from.as_ref());

            let flat = FlatSamples {
                samples,
                layout: SampleLayout {
                    channels: 1,
                    channel_stride: 1,
                    width: width as u32,
                    width_stride: 1,
                    height: height as u32,
                    height_stride: stride,
                },
                color_hint: Some(ColorType::L8),
            };
            flat
        }
    }

    impl From<Image> for FlatSamples<Vec<u8>> {
        fn from(from: Image) -> Self {
            Self::from(&from)
        }
    }

    impl<Container> From<&ImageBuffer<Luma<u8>, Container>> for Image
    where
        Container: Deref<Target = [u8]>,
    {
        fn from(from: &ImageBuffer<Luma<u8>, Container>) -> Self {
            let width = from.width() as usize;
            let height = from.height() as usize;
            let mut image = Self::zeros_alignment(width, height, DEFAULT_ALIGNMENT_U8).unwrap();

            from.enumerate_pixels().for_each(|(x, y, pixel)| {
                let component = pixel.channels()[0];
                image[(x as usize, y as usize)] = component;
            });
            image
        }
    }

    impl<Container> From<ImageBuffer<Luma<u8>, Container>> for Image
    where
        Container: Deref<Target = [u8]>,
    {
        fn from(from: ImageBuffer<Luma<u8>, Container>) -> Self {
            Image::from(&from)
        }
    }

    impl From<&Image> for ImageBuffer<Luma<u8>, Vec<u8>> {
        fn from(from: &Image) -> Self {
            let width = from.width();
            let height = from.height();
            ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
                Luma::from([from[(x as usize, y as usize)]])
            })
        }
    }

    impl From<Image> for ImageBuffer<Luma<u8>, Vec<u8>> {
        fn from(from: Image) -> Self {
            Self::from(&from)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "nalgebra")]
    use nalgebra::MatrixMN;

    #[cfg(feature = "image")]
    use image::{
        flat::{FlatSamples, SampleLayout},
        ColorType, ImageBuffer, Luma,
    };

    #[cfg(feature = "image")]
    fn diagonal_image(width: usize, height: usize) -> Image {
        let mut image = Image::zeros_alignment(width, height, DEFAULT_ALIGNMENT_U8);
        (0..(width.min(height) as usize))
            .into_iter()
            .for_each(|index| {
                image[(index, index)] = 255;
            });
        image
    }

    #[test]
    #[cfg(feature = "image")]
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

    #[test]
    #[cfg(feature = "image")]
    fn convert_flat_samples_vs_image() {
        let width = 64;
        let height = 28;
        let stride = ((width - 1) / DEFAULT_ALIGNMENT_U8 + 1) * DEFAULT_ALIGNMENT_U8;

        let flat_from = {
            let mut samples = Vec::<u8>::new();
            (0..height).into_iter().for_each(|y| {
                let mut row = vec![];
                row.resize(stride as usize, 0);
                (0..width).into_iter().for_each(|x| {
                    if x == y {
                        row[x as usize] = 255;
                    }
                });
                samples.append(&mut row);
            });
            assert_eq!(samples.len(), (height * stride) as usize);

            FlatSamples {
                samples,
                layout: SampleLayout {
                    channels: 1,
                    channel_stride: 1,
                    width: width as u32,
                    width_stride: 1,
                    height: height as u32,
                    height_stride: stride as usize,
                },
                color_hint: Some(ColorType::L8),
            }
        };

        let image = Image::from(&flat_from);
        (0..height)
            .into_iter()
            .flat_map(|y| (0..width).into_iter().map(move |x| (x, y)))
            .for_each(|(x, y)| {
                if x == y {
                    assert_eq!(image[(x as usize, y as usize)], 255);
                } else {
                    assert_eq!(image[(x as usize, y as usize)], 0);
                }
            });

        let flat_to = FlatSamples::from(image);
        assert_eq!(flat_from.color_hint, flat_to.color_hint);
        assert_eq!(flat_from.layout, flat_to.layout);
        assert_eq!(flat_from.samples.len(), flat_to.samples.len());
        assert!({
            flat_from
                .samples
                .iter()
                .zip(flat_from.samples.iter())
                .all(|(lhs, rhs)| lhs == rhs)
        });
    }

    #[test]
    #[cfg(feature = "image")]
    fn convert_image_buffer_vs_image() {
        let width = 120;
        let height = 80;
        let image_buf_from = {
            let mut buf = ImageBuffer::<Luma<u8>, _>::new(width, height);
            (0..(width.min(height))).into_iter().for_each(|idx| {
                buf[(idx, idx)][0] = 255;
            });
            buf
        };

        let image = Image::from(&image_buf_from);
        (0..height)
            .into_iter()
            .flat_map(|y| (0..width).into_iter().map(move |x| (x, y)))
            .for_each(|(x, y)| {
                if x == y {
                    assert_eq!(image[(x as usize, y as usize)], 255);
                } else {
                    assert_eq!(image[(x as usize, y as usize)], 0);
                }
            });

        let image_buf_to = ImageBuffer::from(image);
        assert_eq!(image_buf_from.width(), image_buf_to.width());
        assert_eq!(image_buf_from.height(), image_buf_to.height());
        assert!({
            image_buf_from
                .pixels()
                .zip(image_buf_to.pixels())
                .all(|(lhs, rhs)| lhs == rhs)
        });
    }

    #[test]
    #[cfg(feature = "nalgebra")]
    fn convert_matrix_vs_image() {
        use nalgebra::{U40, U80};
        let matrix_from = MatrixMN::<u8, U80, U40>::new_random();

        let image = Image::from(&matrix_from);
        assert_eq!(matrix_from.nrows(), image.height());
        assert_eq!(matrix_from.ncols(), image.width());
        assert!({
            image
                .samples_iter()
                .all(|(x, y, value)| value == matrix_from[(y, x)])
        });

        let matrix_to = MatrixMN::from(image);
        assert_eq!(matrix_from.nrows(), matrix_to.nrows());
        assert_eq!(matrix_from.ncols(), matrix_to.ncols());
        assert!({
            matrix_from
                .iter()
                .zip(matrix_to.iter())
                .all(|(lhs, rhs)| lhs == rhs)
        });
    }
}
