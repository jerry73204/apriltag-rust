//! AprilTag detector type and its builder.

use crate::{
    detection::Detection,
    error::Error,
    families::{ApriltagFamily, Family},
    image_buf::Image,
    zarray::ZArray,
};
use apriltag_sys as sys;
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
