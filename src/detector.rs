use crate::{families::Family, image_buf::Image, zarray::Zarray};
use apriltag_sys::{
    apriltag_detection_destroy, apriltag_detection_t, apriltag_detector_add_family_bits,
    apriltag_detector_clear_families, apriltag_detector_create, apriltag_detector_destroy,
    apriltag_detector_detect, apriltag_detector_remove_family, apriltag_detector_t, matd_t,
};
#[cfg(feature = "with-nalgebra")]
use nalgebra::{Matrix3, Point2};
use std::{collections::HashSet, convert::TryFrom, os::raw::c_int, ptr::NonNull};

pub trait FromDetectionCenter {
    fn detection_center(center: &[f64; 2]) -> Self;
}

impl FromDetectionCenter for [f64; 2] {
    fn detection_center(center: &[f64; 2]) -> Self {
        center.to_owned()
    }
}

impl FromDetectionCenter for (f64, f64) {
    fn detection_center(center: &[f64; 2]) -> Self {
        let [x, y] = center;
        (*x, *y)
    }
}

#[cfg(feature = "with-nalgebra")]
impl FromDetectionCenter for Point2<f64> {
    fn detection_center(center: &[f64; 2]) -> Self {
        let [x, y] = center.to_owned();
        Point2::new(x, y)
    }
}

pub trait FromDetectionCorners {
    fn detection_corners(corners: &[[f64; 2]; 4]) -> Self;
}

impl FromDetectionCorners for [[f64; 2]; 4] {
    fn detection_corners(corners: &[[f64; 2]; 4]) -> Self {
        corners.to_owned()
    }
}

impl FromDetectionCorners for [(f64, f64); 4] {
    fn detection_corners(corners: &[[f64; 2]; 4]) -> Self {
        <[_; 4]>::try_from(
            corners
                .iter()
                .map(|[x, y]| (*x, *y))
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .unwrap()
    }
}

impl FromDetectionCorners for Vec<[f64; 2]> {
    fn detection_corners(corners: &[[f64; 2]; 4]) -> Self {
        corners.iter().map(|[x, y]| [*x, *y]).collect()
    }
}

impl FromDetectionCorners for Vec<(f64, f64)> {
    fn detection_corners(corners: &[[f64; 2]; 4]) -> Self {
        corners.iter().map(|[x, y]| (*x, *y)).collect()
    }
}

#[cfg(feature = "with-nalgebra")]
impl FromDetectionCorners for [Point2<f64>; 4] {
    fn detection_corners(corners: &[[f64; 2]; 4]) -> Self {
        <[_; 4]>::try_from(
            corners
                .iter()
                .map(|[x, y]| Point2::new(*x, *y))
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .unwrap()
    }
}

#[cfg(feature = "with-nalgebra")]
impl FromDetectionCorners for Vec<Point2<f64>> {
    fn detection_corners(corners: &[[f64; 2]; 4]) -> Self {
        corners
            .iter()
            .map(|[x, y]| Point2::new(*x, *y))
            .collect::<Vec<_>>()
    }
}

pub trait FromDetectionHomographt {
    fn detection_homography(homography: *mut matd_t) -> Self;
}

impl FromDetectionHomographt for [[f64; 3]; 3] {
    // row-major
    fn detection_homography(homography: *mut matd_t) -> Self {
        let as_ref = unsafe { homography.as_ref().unwrap() };
        let nrows = as_ref.nrows;
        let ncols = as_ref.ncols;
        let len = nrows * ncols;
        let data = unsafe { as_ref.data.as_slice(len as usize) };
        [
            [data[0], data[1], data[2]],
            [data[3], data[4], data[5]],
            [data[6], data[7], data[8]],
        ]
    }
}

#[cfg(feature = "with-nalgebra")]
impl FromDetectionHomographt for Matrix3<f64> {
    fn detection_homography(homography: *mut matd_t) -> Self {
        let as_ref = unsafe { homography.as_ref().unwrap() };
        let nrows = as_ref.nrows;
        let ncols = as_ref.ncols;
        let len = nrows * ncols;
        let data = unsafe { as_ref.data.as_slice(len as usize) };
        Matrix3::from_row_slice(data)
    }
}

#[derive(Debug)]
pub struct Detector {
    ptr: NonNull<apriltag_detector_t>,
    families: HashSet<Family>,
}

impl Detector {
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
        let zarray = unsafe {
            let ptr = apriltag_detector_detect(self.ptr.as_ptr(), internal_image.ptr.as_ptr());
            Zarray::<apriltag_detection_t>::from_ptr(NonNull::new(ptr).unwrap())
        };
        todo!();
    }

    pub fn add_family_bits<'a>(&'a mut self, family: Family, bits_corrected: c_int) -> &'a Family {
        unsafe {
            apriltag_detector_add_family_bits(
                self.ptr.as_ptr(),
                family.ptr.as_ptr(),
                bits_corrected,
            );
        }
        self.families.get_or_insert(family)
    }

    fn _remove_family<'a>(&'a mut self, family: &'a Family) {
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

impl Drop for Detector {
    fn drop(&mut self) {
        unsafe { apriltag_detector_destroy(self.ptr.as_ptr()) };
    }
}

pub struct Detection {
    ptr: NonNull<apriltag_detection_t>,
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

    pub fn center<P>(&self) -> P
    where
        P: FromDetectionCenter,
    {
        unsafe { P::detection_center(&self.ptr.as_ref().c) }
    }

    pub fn corners<C>(&self) -> C
    where
        C: FromDetectionCorners,
    {
        unsafe { C::detection_corners(&self.ptr.as_ref().p) }
    }

    pub fn homography<H>(&self) -> H
    where
        H: FromDetectionHomographt,
    {
        unsafe { H::detection_homography(self.ptr.as_ref().H) }
    }
}

impl Drop for Detection {
    fn drop(&mut self) {
        unsafe {
            apriltag_detection_destroy(self.ptr.as_ptr());
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
