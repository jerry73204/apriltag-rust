use crate::matd::MatdRef;
use apriltag_sys as sys;
use std::ptr::NonNull;

#[repr(transparent)]
pub struct Detection {
    ptr: NonNull<sys::apriltag_detection_t>,
}

impl Detection {
    pub fn id(&self) -> usize {
        unsafe { self.ptr.as_ref().id as usize }
    }

    pub fn hamming(&self) -> usize {
        unsafe { self.ptr.as_ref().hamming as usize }
    }

    pub fn decision_margin(&self) -> f32 {
        unsafe { self.ptr.as_ref().decision_margin }
    }

    pub fn center(&self) -> [f64; 2] {
        unsafe { self.ptr.as_ref().c }
    }

    pub fn corners(&self) -> [[f64; 2]; 4] {
        unsafe { self.ptr.as_ref().p }
    }

    pub fn homography<H>(&self) -> MatdRef<'_> {
        unsafe { MatdRef::from_ptr(self.ptr.as_ref().H) }
    }

    pub(crate) unsafe fn from_raw(ptr: *mut sys::apriltag_detection_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }
}

impl Drop for Detection {
    fn drop(&mut self) {
        unsafe {
            sys::apriltag_detection_destroy(self.ptr.as_ptr());
        }
    }
}
