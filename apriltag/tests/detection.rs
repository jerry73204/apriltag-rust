use apriltag::{DetectorBuilder, Family, Image};

#[test]
fn pnm_file_detection() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/test_data/DICT_APRILTAG_16h5-2x2-500-10-0.8-29,12,22,2.pnm"
    );
    let image = Image::from_pnm_file(path).unwrap();

    let mut detector = DetectorBuilder::new()
        .add_family_bits(Family::tag_16h5(), 1)
        .build()
        .expect("Valid builder");

    // Ensure correct parsing of IDs
    let mut ids_found: Vec<_> = detector
        .detect(&image)
        .into_iter()
        .map(|detection| detection.id())
        .collect();
    ids_found.sort_unstable();
    assert_eq!(ids_found, [2, 12, 22, 29]);
}
