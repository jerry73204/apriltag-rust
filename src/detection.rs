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

/// Represent marker detection info for pose estimation.
#[repr(transparent)]
pub struct DetectionInfo {
    pub(crate) ptr: NonNull<sys::apriltag_detection_info_t>,
}

impl DetectionInfo {
    pub fn new(det: &Detection, tagsize: f64, fx: f64, fy: f64, cx: f64, cy: f64) -> Self {
        Self {
            ptr: NonNull::new(&mut sys::apriltag_detection_info_t {
                det: det.ptr.as_ptr(),
                tagsize,
                fx,
                fy,
                cx,
                cy,
            })
            .unwrap(),
        }
    }

    /// Get the underlying detection.
    pub fn det(&self) -> Detection {
        unsafe { Detection::from_raw(self.ptr.as_ref().det) }
    }

    /// Get the marker tagsize.
    pub fn tagsize(&self) -> f64 {
        unsafe { self.ptr.as_ref().tagsize }
    }

    /// Get the camera's focal x length (in pixels).
    pub fn fx(&self) -> f64 {
        unsafe { self.ptr.as_ref().fx }
    }
    /// Get the camera's focal y length (in pixels).
    pub fn fy(&self) -> f64 {
        unsafe { self.ptr.as_ref().fy }
    }
    /// Get the camera's center x (in pixels).
    pub fn cx(&self) -> f64 {
        unsafe { self.ptr.as_ref().cx }
    }
    /// Get the camera's center y (in pixels).
    pub fn cy(&self) -> f64 {
        unsafe { self.ptr.as_ref().cy }
    }

    pub(crate) unsafe fn from_raw(ptr: *mut sys::apriltag_detection_info_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }
}
