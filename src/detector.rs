use crate::{detection::Detection, families::Family, image_buf::Image, zarray::Zarray};
use apriltag_sys as sys;
use std::{mem, os::raw::c_int, ptr::NonNull};

#[derive(Debug)]
pub struct DetectorBuilder {
    families: Vec<(Family, usize)>,
}

impl DetectorBuilder {
    pub fn new() -> Self {
        Self { families: vec![] }
    }

    pub fn add_family_bits(mut self, family: Family, bits_corrected: usize) -> Self {
        self.families.push((family, bits_corrected));
        self
    }

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

#[derive(Debug)]
pub struct Detector {
    pub(crate) ptr: NonNull<sys::apriltag_detector_t>,
}

impl Detector {
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

    pub fn set_debug(&mut self, debug: bool) {
        unsafe {
            self.ptr.as_mut().debug = debug as c_int;
        }
    }

    // pub fn add_family_bits(&mut self, family: Family, bits_corrected: usize) -> SavedFamily<'_> {
    //     unsafe {
    //         sys::apriltag_detector_add_family_bits(
    //             self.ptr.as_ptr(),
    //             family.ptr.as_ptr(),
    //             bits_corrected as c_int,
    //         );
    //         SavedFamily::new(self, family)
    //     }
    // }

    // pub fn remove_family(&mut self, family: SavedFamily<'_>) {
    //     let (detector_ptr, family_ptr) = family.into_raw();
    //     assert_eq!(
    //         self.ptr, detector_ptr,
    //         "cannot remove a family added to another detector"
    //     );

    //     unsafe {
    //         sys::apriltag_detector_remove_family(self.ptr.as_ptr(), family_ptr.as_ptr());
    //     }
    // }

    // pub fn clear_families(&mut self) {
    //     unsafe {
    //         sys::apriltag_detector_clear_families(self.ptr.as_ptr());
    //     }
    // }
}

impl Drop for Detector {
    fn drop(&mut self) {
        unsafe { sys::apriltag_detector_destroy(self.ptr.as_ptr()) };
    }
}
