use crate::detector::Detector;
use apriltag_sys as sys;
use std::{
    ffi::c_void, fmt::Debug, hash::Hash, marker::PhantomData, mem::ManuallyDrop, ptr::NonNull,
};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Family {
    pub(crate) ptr: NonNull<sys::apriltag_family_t>,
}

impl Family {
    pub fn tag_16h5() -> Self {
        let ptr = unsafe { sys::tag16h5_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_25h9() -> Self {
        let ptr = unsafe { sys::tag25h9_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_36h11() -> Self {
        let ptr = unsafe { sys::tag36h11_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_circle_21h7() -> Self {
        let ptr = unsafe { sys::tagCircle21h7_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_circle_49h12() -> Self {
        let ptr = unsafe { sys::tagCircle49h12_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_custom_48h12() -> Self {
        let ptr = unsafe { sys::tagCustom48h12_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_standard_41h12() -> Self {
        let ptr = unsafe { sys::tagStandard41h12_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_standard_52h13() -> Self {
        let ptr = unsafe { sys::tagStandard52h13_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub(crate) unsafe fn into_raw(self) -> *mut sys::apriltag_family_t {
        let ptr = ManuallyDrop::new(self).ptr;
        ptr.as_ptr()
    }

    // pub(crate) unsafe fn from_raw(ptr: *mut sys::apriltag_family_t) -> Self {
    //     Self {
    //         ptr: NonNull::new(ptr).unwrap(),
    //     }
    // }
}

impl Drop for Family {
    fn drop(&mut self) {
        unsafe {
            let ptr = self.ptr.as_ptr();
            libc::free((*ptr).codes as *mut c_void);
            libc::free((*ptr).bit_x as *mut c_void);
            libc::free((*ptr).bit_y as *mut c_void);
            libc::free((*ptr).name as *mut c_void);
            libc::free(ptr as *mut c_void);
        }
    }
}

#[derive(Debug)]
pub struct SavedFamily<'a> {
    pub(crate) detector: &'a Detector,
    pub(crate) ptr: *mut sys::apriltag_family_t,
}

impl<'a> SavedFamily<'a> {
    pub(crate) unsafe fn new(detector: &'a Detector, family: Family) -> Self {
        let ptr = family.into_raw();
        Self { detector, ptr }
    }

    pub(crate) fn into_raw(self) -> (&'a Detector, *mut sys::apriltag_family_t) {
        let Self { detector, ptr } = self;
        (detector, ptr)
    }
}
