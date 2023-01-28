//! The Rusty AprilTag detector.
//!
//! The crate is built on top of [apriltag-sys](apriltag_sys).
//! It provides high level type wrappers on images, detections, and so on.
//!
//! Third-party type conversions are supported by extension crates,
//! including
//!
//! - **apriltag-nalgebra**: Add conversions from/to two dimensional byte matrix in nalgebra crate.
//! - **apriltag-image**: Add conversions from/to image types in image crate.

pub mod detection;
pub mod detector;
pub mod error;
pub mod families;
pub mod image_buf;
pub mod matd;
pub mod pose;
pub mod zarray;

pub use detection::Detection;
pub use detector::{Detector, DetectorBuilder};
pub use error::Error;
pub use families::Family;
pub use image_buf::Image;
pub use matd::MatdRef;
pub use pose::{Pose, PoseEstimation, TagParams};
pub use zarray::ZArray;
