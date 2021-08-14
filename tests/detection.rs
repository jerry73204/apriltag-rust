#[test]
#[cfg(feature = "image")]
fn test_detection() {
    let grayscale_image = {
        let path: std::path::PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "test_data",
            "DICT_APRILTAG_16h5-2x2-500-10-0.8-29,12,22,2.jpg",
        ]
        .iter()
        .collect();
        let file = match image::io::Reader::open(&path) {
            Ok(file) => file,
            Err(error) => {
                panic!(
                    "Reading example data at '{}' failed: {}",
                    path.display(),
                    error
                );
            }
        };
        match file.decode() {
            Ok(img) => img.to_luma8(),
            Err(error) => {
                panic!(
                    "Decoding example data at '{}' failed: {}",
                    path.display(),
                    error
                );
            }
        }
    };

    let mut detector = apriltag::DetectorBuilder::new()
        .add_family_bits(apriltag::Family::tag_16h5(), 1)
        .build()
        .expect("Valid builder");

    // Ensure correct parsing of IDs
    let mut ids_found: Vec<_> = detector
        .detect(&grayscale_image)
        .into_iter()
        .map(|detection| detection.id())
        .collect();
    ids_found.sort_unstable();
    assert_eq!(ids_found, [2, 12, 22, 29]);
}
