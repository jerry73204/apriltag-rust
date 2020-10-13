use crate::{common::*, MatdRef};

/// Estimated pose along with error.
pub struct PoseEstimation {
    pub pose: Pose,
    pub error: f64,
}

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

impl Debug for Pose {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Pose")
            .field("rotation", &self.rotation())
            .field("translation", &self.translation())
            .finish()
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
