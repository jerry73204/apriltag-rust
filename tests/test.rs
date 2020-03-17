#[test]
fn test_link() {
    // This tests that linking worked. If only "cargo build" is issued, it does
    // not guarantee that linking worked. With this test, "cargo test" only
    // passes if the apriltag library was succesfully linked.
    let td: *mut apriltag_sys::apriltag_detector =
        unsafe { apriltag_sys::apriltag_detector_create() };
    unsafe { apriltag_sys::apriltag_detector_destroy(td) };
}
