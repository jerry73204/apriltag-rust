#[test]
fn test_link() {
    // This tests that linking worked. If only "cargo build" is issued, it does
    // not guarantee that linking worked. With this test, "cargo test" only
    // passes if the apriltag library was succesfully linked.
    let td: *mut apriltag_sys::apriltag_detector =
        unsafe { apriltag_sys::apriltag_detector_create() };
    unsafe { apriltag_sys::apriltag_detector_destroy(td) };
}

#[test]
fn test_detection() {
    let reference_image = {
        let path: std::path::PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "tests",
            "test_data",
            "DICT_APRILTAG_16h5-2x2-500-10-0.8-29,12,22,2.jpg",
        ]
        .iter()
        .collect();
        let image_reader = image::io::Reader::open(path).expect("Valid path to test image");
        let image = image_reader.decode().expect("Successful decoding of image");
        image.to_luma8()
    };

    unsafe {
        let family = apriltag_sys::tag16h5_create();
        assert!(!family.is_null());

        let detector = apriltag_sys::apriltag_detector_create();
        assert!(!detector.is_null());
        apriltag_sys::apriltag_detector_add_family_bits(detector, family, 1);
        detector.as_mut().expect("Valid detector").quad_decimate = 1.0;
        detector.as_mut().expect("Valid detector").nthreads = 1;

        let image = apriltag_sys::image_u8_create_stride(
            reference_image.width(),
            reference_image.height(),
            reference_image.width(),
        );
        {
            let width = reference_image.width() as usize;
            let image_data = std::slice::from_raw_parts_mut(
                image.as_mut().expect("Valid allocated image").buf,
                reference_image.width() as usize * reference_image.height() as usize,
            );
            for (x, y, pixel) in image::GenericImageView::pixels(&reference_image) {
                image_data[x as usize + y as usize * width] = pixel.0[0];
            }
        }

        let detections = apriltag_sys::apriltag_detector_detect(detector, image);
        assert_eq!(detections.as_ref().expect("Valid detection array").size, 4);

        apriltag_sys::apriltag_detections_destroy(detections);
        apriltag_sys::image_u8_destroy(image);
        apriltag_sys::apriltag_detector_destroy(detector);
        apriltag_sys::tag16h5_destroy(family);
    }
}
