use crate::MatdRef;
use apriltag_sys as sys;

#[repr(transparent)]
pub struct Pose(pub(crate) sys::apriltag_pose_t);

impl Pose {
    /// Gets the rotation matrix.
    pub fn rotation(&self) -> MatdRef<'_> {
        unsafe { MatdRef::from_ptr(self.0.R) }
    }

    /// Gets the translation matrix.
    pub fn translation(&self) -> MatdRef<'_> {
        unsafe { MatdRef::from_ptr(self.0.t) }
    }
}

impl Drop for Pose {
    fn drop(&mut self) {
        unsafe {
            sys::matd_destroy(self.0.R);
            sys::matd_destroy(self.0.t);
        }
    }
}
