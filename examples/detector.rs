use anyhow::Result;

#[cfg(feature = "image")]
mod simple_detector {
    use anyhow::Result;
    use apriltag::{DetectorBuilder, Family};
    use argh::FromArgs;
    use std::path::PathBuf;

    #[derive(Debug, Clone, FromArgs)]
    /// Simple AprilTag detector.
    struct Args {
        #[argh(positional)]
        /// input files.
        pub input_files: Vec<PathBuf>,
    }

    pub fn _main() -> Result<()> {
        let Args { input_files } = argh::from_env();

        if input_files.is_empty() {
            eprintln!("no input files");
            return Ok(());
        }

        let mut detector = DetectorBuilder::new()
            .add_family_bits(Family::tag_16h5(), 1)
            .build()
            .unwrap();

        for path in input_files.into_iter() {
            let image = image::open(&path)?;
            let detections = detector.detect(image.to_luma());

            println!("# image {}", path.display());

            detections.into_iter().enumerate().for_each(|(index, det)| {
                println!(
                    "- detection {}\n\
                     id: {}\n\
                     hamming: {}\n\
                     decision_margin: {}\n\
                     center: {:?}\n\
                     corners: {:?}\n\
                     homography: {:?}\n\
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
fn main() -> Result<()> {
    simple_detector::_main()
}

#[cfg(not(feature = "image"))]
fn main() -> Result<()> {
    panic!(r#"please enable the "image" feature to run the example"#);
}
