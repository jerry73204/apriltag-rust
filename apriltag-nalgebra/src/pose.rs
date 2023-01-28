use apriltag::Pose;
use nalgebra::{Isometry3, MatrixView3, MatrixView3x1, Translation3, UnitQuaternion};

pub trait PoseExt {
    fn to_na(&self) -> Isometry3<f64>;
}

impl PoseExt for Pose {
    fn to_na(&self) -> Isometry3<f64> {
        let rotation = self.rotation();
        let translation = self.translation();

        let rotation =
            UnitQuaternion::from_matrix(&MatrixView3::from_slice(rotation.data()).transpose());

        let translation: Translation3<f64> = MatrixView3x1::from_slice(translation.data())
            .into_owned()
            .into();

        Isometry3::from_parts(translation, rotation)
    }
}
