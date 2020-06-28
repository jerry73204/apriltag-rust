//! Tag detection types.

use crate::matd::MatdRef;
use apriltag_sys as sys;
use std::ptr::NonNull;

/// Represent a marker detection outcome.
#[repr(transparent)]
pub struct Detection {
    ptr: NonNull<sys::apriltag_detection_t>,
}

impl Detection {
    /// Get the marker ID.
    pub fn id(&self) -> usize {
        unsafe { self.ptr.as_ref().id as usize }
    }

    /// Get the Hamming distance to the target tag.
    pub fn hamming(&self) -> usize {
        unsafe { self.ptr.as_ref().hamming as usize }
    }

    /// Indicate the _goodness_ of the detection.
    pub fn decision_margin(&self) -> f32 {
        unsafe { self.ptr.as_ref().decision_margin }
    }

    /// Get the center coordinates in form of `[x, y]`.
    pub fn center(&self) -> [f64; 2] {
        unsafe { self.ptr.as_ref().c }
    }

    /// Get the corner coordinates in form of `[[x, y]; 4]`.
    pub fn corners(&self) -> [[f64; 2]; 4] {
        unsafe { self.ptr.as_ref().p }
    }

    /// Get the homography matrix.
    pub fn homography(&self) -> MatdRef<'_> {
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
