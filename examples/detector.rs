use failure::Fallible;

#[cfg(feature = "image")]
mod simple_detector {
    use apriltag::{DetectorBuilder, Family};
    use argh::FromArgs;
    use failure::Fallible;
    use std::path::PathBuf;

    #[derive(Debug, Clone, FromArgs)]
    /// Simple AprilTag detector.
    struct Args {
        #[argh(positional)]
        /// input files.
        pub input_files: Vec<PathBuf>,
    }

    pub fn _main() -> Fallible<()> {
        let Args { input_files } = argh::from_env();

        if input_files.is_empty() {
            eprintln!("no input files");
            return Ok(());
        }

        let mut detector = DetectorBuilder::new()
            .add_family_bits(Family::tag_standard_52h13(), 1)
            .build()
            .unwrap();

        for path in input_files.into_iter() {
            let image = image::open(&path)?;
            let detections = detector.detect(image.to_luma());

            println!("# image {}", path.display());

            detections.into_iter().enumerate().for_each(|(index, det)| {
                println!(
                    "- detection {}\
                     id\t{}\
                     hamming\t{}\
                     decision_margin\t{}\
                     center\t{:?}\
                     corners\t{:?}\
                     homography\t{:?}\

                     ",
                    index,
                    det.id(),
                    det.hamming(),
                    det.decision_margin(),
                    det.center(),
                    det.corners(),
                    det.homography().data()
                );
            });
        }
        Ok(())
    }
}

#[cfg(feature = "image")]
fn main() -> Fallible<()> {
    simple_detector::_main()
}

#[cfg(not(feature = "image"))]
fn main() -> Fallible<()> {
    panic!(r#"please enable the "image" feature to run the example"#);
}
