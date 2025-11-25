//! AprilTag detector type and its builder.

use crate::{
    detection::Detection,
    error::Error,
    families::{ApriltagFamily, Family},
    image_buf::Image,
    zarray::ZArray,
};
use apriltag_sys as sys;
use measurements::angle::Angle;
use noisy_float::prelude::R32;
use std::{ffi::c_int, mem::ManuallyDrop, ptr::NonNull};

/// The detector builder that creates [Detector].
#[derive(Debug)]
pub struct DetectorBuilder {
    families: Vec<(Family, usize)>,
}

impl DetectorBuilder {
    /// Create a builder instance.
    pub fn new() -> Self {
        Self { families: vec![] }
    }

    /// Append a tag family.
    ///
    /// The method must be called at least once.
    pub fn add_family_bits<F>(mut self, family: F, bits_corrected: usize) -> Self
    where
        F: Into<Family>,
    {
        self.families.push((family.into(), bits_corrected));
        self
    }

    /// Create a [Detector] instance.
    ///
    /// If [add_family_bits](DetectorBuilder::add_family_bits) is never called.
    /// it returns `None`.
    pub fn build(self) -> Result<Detector, Error> {
        if self.families.is_empty() {
            return Err(Error::CreateDetectorError {
                reason: "There is not family set for the detector. \
                         Did you call add_family_bits()?"
                    .to_string(),
            });
        }

        let detector_ptr = unsafe { sys::apriltag_detector_create() };
        let detector_ptr =
            NonNull::new(detector_ptr).ok_or_else(|| Error::CreateDetectorError {
                reason: "apriltag_detector_create() failed".to_string(),
            })?;
        for (family, bits_corrected) in self.families {
            unsafe {
                let family_ptr = family.into_raw();
                sys::apriltag_detector_add_family_bits(
                    detector_ptr.as_ptr(),
                    family_ptr,
                    bits_corrected as c_int,
                );
            }
        }

        Ok(Detector { ptr: detector_ptr })
    }
}

impl Default for DetectorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// The marker detector.
#[derive(Debug)]
#[repr(transparent)]
pub struct Detector {
    pub(crate) ptr: NonNull<sys::apriltag_detector_t>,
}

impl Detector {
    pub fn builder() -> DetectorBuilder {
        DetectorBuilder::new()
    }

    /// Run detection on the input image.
    pub fn detect(&mut self, image: &Image) -> Vec<Detection> {
        let detections = unsafe {
            let ptr = sys::apriltag_detector_detect(self.ptr.as_ptr(), image.ptr.as_ptr());
            let zarray = ZArray::<*mut sys::apriltag_detection_t>::from_raw(ptr);
            let detections = zarray
                .iter()
                .cloned()
                .map(|ptr| Detection::from_raw(ptr))
                .collect::<Vec<_>>();
            detections
        };
        detections
    }

    /// Set the number of threads used for detection.
    pub fn set_thread_number(&mut self, num_threads: u8) {
        unsafe {
            self.ptr.as_mut().nthreads = num_threads as c_int;
        }
    }

    /// Decimate the input image.
    ///
    /// The detection of quads can be done on a lower-resolution image, improving speed at a cost of pose accuracy and a slight decrease in detection rate.
    /// Decoding the binary payload is still done at full resolution.
    pub fn set_decimation(&mut self, decimation: f32) {
        unsafe {
            self.ptr.as_mut().quad_decimate = decimation;
        }
    }

    /// Set the standard deviation in pixels for Gaussian blur applied to the segmented image for quad detection.
    /// Very noisy images benefit from non-zero values (e.g. 0.8).
    pub fn set_sigma(&mut self, sigma: f32) {
        unsafe {
            self.ptr.as_mut().quad_sigma = sigma;
        }
    }

    /// Enable refinement of edges.
    ///
    /// When enabled, the edges of the each quad are adjusted to "snap to" strong gradients nearby.
    /// This is useful when decimation is employed, as it can increase the quality of the initial quad estimate substantially.
    /// Both very computationally inexpensive and generally recommended to be enabled. The option is ignored if decimation is disabled.
    pub fn set_refine_edges(&mut self, refine_edges: bool) {
        unsafe {
            self.ptr.as_mut().refine_edges = refine_edges;
        }
    }

    /// Set the amount of sharpening applied to the decoded images.
    ///
    /// This can help decode small tags but may or may not help in odd lighting conditions or low light conditions.
    /// The default value is 0.25.
    pub fn set_shapening(&mut self, shapening: f64) {
        unsafe {
            self.ptr.as_mut().decode_sharpening = shapening;
        }
    }

    /// Enable or disable the debugging message.
    ///
    /// It is disabled by default.
    pub fn set_debug(&mut self, debug: bool) {
        unsafe {
            self.ptr.as_mut().debug = debug;
        }
    }

    /// Set various thresholds for detecting quads as candidates for further processing.
    pub fn set_thresholds(&mut self, thresholds: QuadThresholds) {
        unsafe {
            self.ptr.as_mut().qtp = thresholds.to_c_params();
        }
    }

    /// Creates an instance from pointer.
    ///
    /// The pointer will be managed by the type. Do not run manual deallocation on the pointer.
    /// Panics if the pointer is null.
    ///
    /// # Safety
    /// The method is safe when the pointer was created by [apriltag_detector_create](sys::apriltag_detector_create).
    pub unsafe fn from_raw(ptr: *mut sys::apriltag_detector_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Returns the underlying pointer.
    pub fn into_raw(self) -> NonNull<sys::apriltag_detector_t> {
        ManuallyDrop::new(self).ptr
    }
}

impl Drop for Detector {
    fn drop(&mut self) {
        unsafe { sys::apriltag_detector_destroy(self.ptr.as_ptr()) };
    }
}

/// The adjustable theshold for detecting candidates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuadThresholds {
    /// Enforce a minimal number of pixels per candidate.
    pub min_cluster_pixels: u32,

    /// Specify the number of pixels to consider when segmenting a
    /// group of pixels into a quad.
    pub max_maxima_number: u32,

    /// Reject quads where pairs of edges have angles that are close
    /// to a straight lines.
    pub min_angle: Angle,

    /// Reject quads where pairs of edges have angles that are close
    /// to a 180 degrees.
    pub min_opposite_angle: Angle,

    /// Specify the maximal mean squared error when fittings lines to
    /// the contour. Useful for performance evaluation.
    pub max_mse: R32,

    /// Specify the minimal difference in grey intensity between the
    /// white model and black model.
    pub min_white_black_diff: u8,

    /// Enable deglitching of images useful for very noisy images.
    pub deglitch: bool,
}

impl QuadThresholds {
    fn to_c_params(&self) -> sys::apriltag_quad_thresh_params {
        let Self {
            min_cluster_pixels,
            max_maxima_number,
            min_angle,
            min_opposite_angle,
            max_mse,
            min_white_black_diff,
            deglitch,
        } = *self;

        sys::apriltag_quad_thresh_params {
            min_cluster_pixels: min_cluster_pixels as c_int,
            max_nmaxima: max_maxima_number as c_int,
            critical_rad: min_angle.as_radians() as f32,
            cos_critical_rad: min_opposite_angle.as_radians() as f32,
            max_line_fit_mse: max_mse.raw(),
            min_white_black_diff: min_white_black_diff as c_int,
            deglitch: deglitch as c_int,
        }
    }
}
