use crate::{
    families::{Family, SavedFamily},
    image_buf::Image,
    matd::MatdRef,
    zarray::Zarray,
};
use apriltag_sys as sys;
#[cfg(feature = "nalgebra")]
use nalgebra::{Matrix3, Point2};
use std::{collections::HashSet, convert::TryFrom, os::raw::c_int, ptr::NonNull};

#[derive(Debug)]
pub struct Detector {
    ptr: NonNull<sys::apriltag_detector_t>,
}

impl Detector {
    pub fn new() -> Self {
        let ptr = unsafe { NonNull::new(sys::apriltag_detector_create()).unwrap() };
        Self { ptr }
    }

    pub fn detect<M>(&mut self, image: M)
    where
        M: Into<Image>,
    {
        let image: Image = image.into();
        let zarray = unsafe {
            let ptr = sys::apriltag_detector_detect(self.ptr.as_ptr(), image.ptr.as_ptr());
            Zarray::<sys::apriltag_detection_t>::from_ptr(NonNull::new(ptr).unwrap())
        };
        todo!();
    }

    pub fn add_family_bits<'a>(
        &'a mut self,
        family: Family,
        bits_corrected: c_int,
    ) -> SavedFamily<'a> {
        unsafe {
            sys::apriltag_detector_add_family_bits(
                self.ptr.as_ptr(),
                family.ptr.as_ptr(),
                bits_corrected,
            );
            SavedFamily::new(self, family)
        }
    }

    pub fn remove_family<'a>(&'a mut self, family: SavedFamily<'a>) {
        let (detector, family_ptr) = family.into_raw();
        assert_eq!(
            self.ptr, detector.ptr,
            "cannot remove a family added to another detector"
        );

        unsafe {
            sys::apriltag_detector_remove_family(self.ptr.as_ptr(), family_ptr);
        }
    }

    pub fn clear_families(&mut self) {
        unsafe {
            sys::apriltag_detector_clear_families(self.ptr.as_ptr());
        }
    }
}

impl Drop for Detector {
    fn drop(&mut self) {
        unsafe { sys::apriltag_detector_destroy(self.ptr.as_ptr()) };
    }
}

pub struct Detection {
    ptr: NonNull<sys::apriltag_detection_t>,
}

impl Detection {
    pub fn id(&self) -> usize {
        unsafe { self.ptr.as_ref().id as usize }
    }

    pub fn hamming(&self) -> usize {
        unsafe { self.ptr.as_ref().hamming as usize }
    }

    pub fn decision_margin(&self) -> f32 {
        unsafe { self.ptr.as_ref().decision_margin }
    }

    pub fn center(&self) -> [f64; 2] {
        unsafe { self.ptr.as_ref().c }
    }

    pub fn corners(&self) -> [[f64; 2]; 4] {
        unsafe { self.ptr.as_ref().p }
    }

    pub fn homography<H>(&self) -> MatdRef<'_> {
        unsafe { MatdRef::from_ptr(self.ptr.as_ref().H) }
    }
}

impl Drop for Detection {
    fn drop(&mut self) {
        unsafe {
            sys::apriltag_detection_destroy(self.ptr.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apriltag_detector() {
        let mut detector = Detector::new();
        let family = Family::tag_16h5();
        detector.add_family_bits(family, 1);
    }
}
