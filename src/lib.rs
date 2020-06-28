//! The Rusty AprilTag detector.
//!
//! The crate is built on top of [apriltag-sys](apriltag_sys).
//! It provides high level type wrappers on images, detections, and so on.
//!
//! The feature flags control the supported third-party type conversions. It includes
//! - **nalgebra**: Add conversions from/to two dimensional byte matrix in nalgebra crate.
//! - **image**: Add conversions from/to image types in image crate.

pub mod detection;
pub mod detector;
pub mod families;
pub mod image_buf;
pub mod matd;
mod zarray;

pub use detection::Detection;
pub use detector::{Detector, DetectorBuilder};
pub use families::Family;
pub use image_buf::Image;
pub use matd::MatdRef;
