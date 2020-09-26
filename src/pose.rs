use crate::{detection::DetectionInfo, MatdRef};
use apriltag_sys as sys;
use std::ptr::NonNull;

#[repr(transparent)]
pub struct Pose {
    pub(crate) ptr: NonNull<sys::apriltag_pose_t>,
}

impl Pose {
    pub fn R(&self) -> MatdRef<'_> {
        unsafe { MatdRef::from_ptr(self.ptr.as_ref().R) }
    }

    pub fn t(&self) -> MatdRef<'_> {
        unsafe { MatdRef::from_ptr(self.ptr.as_ref().t) }
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut sys::apriltag_pose_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).unwrap()
        }
    }
}

pub fn estimate_tag_pose(info: &mut DetectionInfo) -> Pose {
    unsafe {
        let pose_ptr = &mut sys::apriltag_pose_t {
            R: sys::matd_create(0, 0),
            t: sys::matd_create(0, 0)
        } as *mut _;
    
        sys::estimate_tag_pose(info.ptr.as_ptr(), pose_ptr);
    
        return Pose::from_ptr(pose_ptr);
    }
}
