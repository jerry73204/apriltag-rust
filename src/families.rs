//! Dictionary of tag families.
//!
//! It provides the dictionary of pre-generated tag families.
//! The images of pre-generated tags can be found at the official repositoy
//! [https://github.com/AprilRobotics/apriltag-imgs](https://github.com/AprilRobotics/apriltag-imgs).

use crate::common::*;

/// Represent a family of pre-generated tags.
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Family {
    pub(crate) ptr: NonNull<sys::apriltag_family_t>,
}

impl Family {
    /// Create Tag16h5 family.
    pub fn tag_16h5() -> Self {
        let ptr = unsafe { sys::tag16h5_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create Tag25h9 family.
    pub fn tag_25h9() -> Self {
        let ptr = unsafe { sys::tag25h9_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create Tag36h11 family.
    pub fn tag_36h11() -> Self {
        let ptr = unsafe { sys::tag36h11_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create TagCircle21h7 family.
    pub fn tag_circle_21h7() -> Self {
        let ptr = unsafe { sys::tagCircle21h7_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create TagCircle49h12 family.
    pub fn tag_circle_49h12() -> Self {
        let ptr = unsafe { sys::tagCircle49h12_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create TagCustom48h12 family.
    pub fn tag_custom_48h12() -> Self {
        let ptr = unsafe { sys::tagCustom48h12_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create TagStandard41h12 family.
    pub fn tag_standard_41h12() -> Self {
        let ptr = unsafe { sys::tagStandard41h12_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create TagStandard52h13 family.
    pub fn tag_standard_52h13() -> Self {
        let ptr = unsafe { sys::tagStandard52h13_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub(crate) unsafe fn into_raw(self) -> NonNull<sys::apriltag_family_t> {
        ManuallyDrop::new(self).ptr
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
