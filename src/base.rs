use crate::{families::AprilTagFamily, image_buf::Image};
use apriltag_sys::{
    apriltag_detector_add_family_bits, apriltag_detector_clear_families, apriltag_detector_create,
    apriltag_detector_destroy, apriltag_detector_detect, apriltag_detector_remove_family,
    apriltag_detector_t,
};
use std::{collections::HashSet, os::raw::c_int, ptr::NonNull};

#[derive(Debug)]
pub struct AprilTagDetector {
    ptr: NonNull<apriltag_detector_t>,
    families: HashSet<AprilTagFamily>,
}

impl AprilTagDetector {
    pub fn new() -> Self {
        let ptr = unsafe { NonNull::new(apriltag_detector_create()).unwrap() };
        Self {
            ptr,
            families: HashSet::new(),
        }
    }

    pub fn detect<M>(&mut self, image: M)
    where
        M: Into<Image>,
    {
        let internal_image: Image = image.into();
        unsafe { apriltag_detector_detect(self.ptr.as_ptr(), internal_image.ptr.as_ptr()) };
        todo!();
    }

    pub fn add_family_bits<'a>(
        &'a mut self,
        family: AprilTagFamily,
        bits_corrected: c_int,
    ) -> &'a AprilTagFamily {
        unsafe {
            apriltag_detector_add_family_bits(
                self.ptr.as_ptr(),
                family.ptr.as_ptr(),
                bits_corrected,
            );
        }
        self.families.get_or_insert(family)
    }

    fn _remove_family<'a>(&'a mut self, family: &'a AprilTagFamily) {
        unsafe {
            apriltag_detector_remove_family(self.ptr.as_ptr(), family.ptr.as_ptr());
        }
        self.families.remove(family);
    }

    pub fn clear_families(&mut self) {
        unsafe {
            apriltag_detector_clear_families(self.ptr.as_ptr());
        }
        self.families.clear();
    }
}

impl Drop for AprilTagDetector {
    fn drop(&mut self) {
        unsafe { apriltag_detector_destroy(self.ptr.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apriltag_detector() {
        let mut detector = AprilTagDetector::new();
        let family = AprilTagFamily::tag_16h5();
        detector.add_family_bits(family, 1);
    }
}
