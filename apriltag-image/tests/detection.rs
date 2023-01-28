use apriltag::{Detector, Family, Image};
use apriltag_image::ImageExt;

#[test]
fn jpg_file_detection() {
    let image = {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_data/DICT_APRILTAG_16h5-2x2-500-10-0.8-29,12,22,2.jpg"
        );
        let file = image::io::Reader::open(path).unwrap_or_else(|error| {
            panic!("Failed to read example data at '{path}': {error}");
        });
        let image = file.decode().unwrap_or_else(|error| {
            panic!("Decoding example data at '{path}' failed: {error}");
        });
        Image::from_image_buffer(&image.to_luma8())
    };

    let mut detector = Detector::builder()
        .add_family_bits(Family::tag_16h5(), 1)
        .build()
        .unwrap();

    // Ensure correct parsing of IDs
    let mut ids_found: Vec<_> = detector
        .detect(&image)
        .into_iter()
        .map(|detection| detection.id())
        .collect();
    ids_found.sort_unstable();
    assert_eq!(ids_found, [2, 12, 22, 29]);
}
