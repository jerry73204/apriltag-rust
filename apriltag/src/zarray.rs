//! The heap allocated array used by AprilTag library.

use apriltag_sys as sys;
use std::{
    ffi::c_char,
    marker::PhantomData,
    mem::{self, ManuallyDrop},
    ops::{Deref, DerefMut, Index, IndexMut},
    os::raw::c_void,
    ptr::NonNull,
    slice,
};

/// A heap allocated array.
#[derive(Debug)]
#[repr(transparent)]
pub struct ZArray<T> {
    ptr: NonNull<sys::zarray_t>,
    _phantom: PhantomData<T>,
}

impl<T> ZArray<T> {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        unsafe { self.ptr.as_ref().size as usize }
    }

    pub fn iter(&self) -> ZArrayIter<T> {
        ZArrayIter {
            zarray: self,
            len: self.len(),
            index: 0,
        }
    }

    /// Creates an instances from raw pointer.
    ///
    /// # Safety
    /// The method is safe when the pointer was created by [apriltag_detector_detect](sys::apriltag_detector_detect).
    pub unsafe fn from_raw(ptr: *mut sys::zarray_t) -> Self {
        let ptr = NonNull::new(ptr).expect("please report bug");
        assert_eq!(ptr.as_ref().el_sz, mem::size_of::<T>(), "please report bug");
        Self {
            ptr,
            _phantom: PhantomData,
        }
    }

    /// Returns the underlying raw pointer.
    pub fn into_raw(self) -> NonNull<sys::zarray_t> {
        ManuallyDrop::new(self).ptr
    }
}

impl<T> Deref for ZArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let as_ref = self.ptr.as_ref();
            slice::from_raw_parts(as_ref.data as *const T, as_ref.size as usize)
        }
    }
}

impl<T> DerefMut for ZArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let as_mut = self.ptr.as_mut();
            slice::from_raw_parts_mut(as_mut.data as *mut T, as_mut.size as usize)
        }
    }
}

impl<T> AsRef<[T]> for ZArray<T> {
    fn as_ref(&self) -> &[T] {
        self.deref()
    }
}

impl<T> AsMut<[T]> for ZArray<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.deref_mut()
    }
}

impl<T> Index<usize> for ZArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl<T> IndexMut<usize> for ZArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl<T> Clone for ZArray<T> {
    fn clone(&self) -> Self {
        let ptr = unsafe {
            let from_ptr = self.ptr.as_ptr();
            let to_ptr = libc::calloc(1, mem::size_of::<sys::zarray_t>()) as *mut sys::zarray_t;

            let sys::zarray_t {
                el_sz,
                size,
                alloc,
                data: from_data,
            } = *from_ptr;
            assert!(size <= alloc);
            assert_eq!(el_sz, mem::size_of::<T>());

            let to_data = libc::malloc(alloc as usize * el_sz);
            libc::memcpy(to_data, from_data as *mut c_void, size as usize * el_sz);

            *to_ptr.as_mut().unwrap() = sys::zarray_t {
                el_sz,
                size,
                alloc,
                data: to_data as *mut c_char,
            };
            to_ptr
        };

        Self {
            ptr: NonNull::new(ptr).unwrap(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Drop for ZArray<T> {
    fn drop(&mut self) {
        unsafe {
            let data_ptr = self.ptr.as_mut().data;
            if !data_ptr.is_null() {
                libc::free(data_ptr as *mut c_void);
            }
            libc::free(self.ptr.as_ptr() as *mut c_void);
        }
    }
}

/// An iterator to [ZArray].
#[derive(Debug, Clone)]
pub struct ZArrayIter<'a, T> {
    zarray: &'a ZArray<T>,
    len: usize,
    index: usize,
}

impl<'a, T> Iterator for ZArrayIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let index = self.index;
            self.index += 1;
            Some(&self.zarray[index])
        } else {
            None
        }
    }
}
