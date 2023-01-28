//! Adds image conversion from/to [nalgebra] crate for [apriltag] crate.

mod image;
mod matd;
mod pose;

pub use image::ImageExt;
pub use matd::MatdRefExt;
pub use nalgebra;
pub use pose::PoseExt;

pub mod prelude {
    pub use crate::{ImageExt as _, MatdRefExt as _, PoseExt as _};
}
