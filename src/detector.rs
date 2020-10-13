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

    /// Enable or disable the debugging message.
    ///
    /// It is disabled by default.
    pub fn set_debug(&mut self, debug: bool) {
        unsafe {
            self.ptr.as_mut().debug = debug as c_int;
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
