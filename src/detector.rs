//! AprilTag detector type and its builder.

use crate::{common::*, detection::Detection, families::Family, image_buf::Image, zarray::Zarray};

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
    pub fn add_family_bits(mut self, family: Family, bits_corrected: usize) -> Self {
        self.families.push((family, bits_corrected));
        self
    }

    /// Create a [Detector] instance.
    ///
    /// If [add_family_bits](DetectorBuilder::add_family_bits) is never called.
    /// it returns `None`.
    pub fn build(self) -> Option<Detector> {
        if self.families.is_empty() {
            return None;
        }

        let detector_ptr = unsafe { NonNull::new(sys::apriltag_detector_create()).unwrap() };
        for (family, bits_corrected) in self.families.into_iter() {
            unsafe {
                let family_ptr = family.into_raw();
                sys::apriltag_detector_add_family_bits(
                    detector_ptr.as_ptr(),
                    family_ptr.as_ptr(),
                    bits_corrected as c_int,
                );
            }
        }

        Some(Detector { ptr: detector_ptr })
    }
}

impl Default for DetectorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// The marker detector.
#[derive(Debug)]
pub struct Detector {
    pub(crate) ptr: NonNull<sys::apriltag_detector_t>,
}

impl Detector {
    /// Run detection on the input image.
    pub fn detect<M>(&mut self, image: M) -> Vec<Detection>
    where
        M: Into<Image>,
    {
        let image: Image = image.into();
        let detections = unsafe {
            let ptr = sys::apriltag_detector_detect(self.ptr.as_ptr(), image.ptr.as_ptr());
            mem::drop(image);
            let zarray = Zarray::<*mut sys::apriltag_detection_t>::from_raw(ptr);
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
            self.ptr.as_mut().refine_edges = refine_edges as c_int;
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
            self.ptr.as_mut().debug = debug as c_int;
        }
    }

    /// Set various thresholds for detecting quads as candidates for further processing.
    pub fn set_thresholds(&mut self, thresholds: QuadThresholds) {
        unsafe {
            self.ptr.as_mut().qtp = thresholds.into();
        }
    }

    /// Creates an instance from pointer.
    ///
    /// The pointer will be managed by the type. Do not run manual deallocation on the pointer.
    /// Panics if the pointer is null.
    pub unsafe fn from_raw(ptr: *mut sys::apriltag_detector_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Returns the underlying pointer.
    pub unsafe fn into_raw(self) -> NonNull<sys::apriltag_detector_t> {
        ManuallyDrop::new(self).ptr
    }
}

impl Drop for Detector {
    fn drop(&mut self) {
        unsafe { sys::apriltag_detector_destroy(self.ptr.as_ptr()) };
    }
}

/// An angle used a threshold for detecting candidates.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(f32);

impl Angle {
    /// Accept all angles.
    pub fn accept_all_candidates() -> Self {
        Self(0.0)
    }

    /// Construct the (minimal) angle from radians.
    ///
    /// # Example
    /// ```
    /// use apriltag::detector::Angle;
    ///
    /// let angle_1 = Angle::from_radians(1.42);
    /// let angle_2 = Angle::from_radians(std::f32::consts::PI * 2.0 + 1.42);
    ///
    /// // Ensure correct wrapping despite potential floating point errors
    /// assert!((angle_1.to_radians() - angle_2.to_radians()).abs() < 0.00001);
    /// ```
    pub fn from_radians(radians: f32) -> Self {
        const FULL_TURN: f32 = std::f32::consts::PI * 2.0;
        let rem = radians % FULL_TURN;
        Self(if rem < 0.0 { rem + FULL_TURN } else { rem })
    }

    /// Construct the (minimal) angle from degrees.
    pub fn from_degrees(degrees: f32) -> Self {
        Self(degrees.to_radians())
    }

    /// Interpret the angle as radians.
    pub fn to_radians(self) -> f32 {
        self.0
    }
}

/// The adjustable theshold for detecting candidates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuadThresholds {
    /// Enforce a minimal number of pixels per candidate.
    pub min_cluster_pixels: u32,
    /// Specify the number of pixels to consider when segmenting a group of pixels into a quad.
    pub max_maxima_number: u32,
    /// Reject quads where pairs of edges have angles that are close to a straight lines.
    pub min_angle: Angle,
    /// Reject quads where pairs of edges have angles that are close to a 180 degrees.
    pub min_opposite_angle: Angle,
    /// Specify the maximal mean squared error when fittings lines to the contour. Useful for performance evaluation.
    pub max_mse: f32,
    /// Specify the minimal difference in grey intensity between the white model and black model.
    pub min_white_black_diff: u8,
    /// Enable deglitching of images useful for very noisy images.
    pub deglitch: bool,
}

impl From<QuadThresholds> for sys::apriltag_quad_thresh_params {
    fn from(thresholds: QuadThresholds) -> Self {
        sys::apriltag_quad_thresh_params {
            min_cluster_pixels: thresholds.min_cluster_pixels as c_int,
            max_nmaxima: thresholds.max_maxima_number as c_int,
            critical_rad: thresholds.min_angle.to_radians(),
            cos_critical_rad: thresholds.min_opposite_angle.to_radians(),
            max_line_fit_mse: thresholds.max_mse,
            min_white_black_diff: thresholds.min_white_black_diff as c_int,
            deglitch: thresholds.deglitch as c_int,
        }
    }
}
