use apriltag_sys::{
    apriltag_family_t, tag16h5_create, tag25h9_create, tag36h11_create, tagCircle21h7_create,
    tagCircle49h12_create, tagCustom48h12_create, tagStandard41h12_create, tagStandard52h13_create,
};
use std::ptr::NonNull;
use std::{ffi::c_void, fmt::Debug, hash::Hash};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct AprilTagFamily {
    pub(crate) ptr: NonNull<apriltag_family_t>,
}

impl AprilTagFamily {
    pub fn tag_16h5() -> Self {
        let ptr = unsafe { tag16h5_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_25h9() -> Self {
        let ptr = unsafe { tag25h9_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_36h11() -> Self {
        let ptr = unsafe { tag36h11_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_circle_21h7() -> Self {
        let ptr = unsafe { tagCircle21h7_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_circle_49h12() -> Self {
        let ptr = unsafe { tagCircle49h12_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_custom_48h12() -> Self {
        let ptr = unsafe { tagCustom48h12_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_standard_41h12() -> Self {
        let ptr = unsafe { tagStandard41h12_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub fn tag_standard_52h13() -> Self {
        let ptr = unsafe { tagStandard52h13_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }
}

impl Drop for AprilTagFamily {
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
