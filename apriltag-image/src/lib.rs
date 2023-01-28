//! Adds image conversion from/to [image] crate for [apriltag] crate.
//!
//! # Example
//!
//! ```rust
//! use apriltag::{Detector, Family, Image};
//! use apriltag_image::prelude::*;
//!
//! fn main() -> anyhow::Result<()> {
//!     let path = "test_data/DICT_APRILTAG_16h5-2x2-500-10-0.8-29,12,22,2.jpg";
//!     let reader = image::io::Reader::open(path)?;
//!     let image_buf = reader.decode()?.to_luma8();
//!     let image = Image::from_image_buffer(&image_buf);
//!     let mut detector = Detector::builder()
//!         .add_family_bits(Family::tag_16h5(), 1)
//!         .build()?;
//!     let detections = detector.detect(&image);
//!     Ok(())
//! }
//! ```

mod image_buf;

pub use crate::image_buf::ImageExt;
pub use image;

pub mod prelude {
    pub use crate::ImageExt as _;
}
