use anyhow::Result;

#[cfg(feature = "full")]
mod simple_detector {
    use anyhow::{ensure, Context, Error, Result};
    use apriltag::{DetectorBuilder, Family, TagParams};
    use argh::FromArgs;
    use std::{path::PathBuf, str::FromStr};

    #[derive(Debug, Clone, FromArgs)]
    /// Simple AprilTag detector.
    struct Args {
        #[argh(option, default = "\"tag16h5\".to_string()")]
        /// family name.
        pub family: String,
        #[argh(option)]
        /// optional tag parameters in format "tagsize,fx,fy,cx,cy".
        pub tag_params: Option<TagParamsArg>,
        #[argh(positional)]
        /// input files.
        pub input_files: Vec<PathBuf>,
    }

    #[derive(Debug, Clone)]
    struct TagParamsArg {
        pub tagsize: f64,
        pub fx: f64,
        pub fy: f64,
        pub cx: f64,
        pub cy: f64,
    }

    impl From<TagParamsArg> for TagParams {
        fn from(arg: TagParamsArg) -> Self {
            let TagParamsArg {
                tagsize,
                fx,
                fy,
                cx,
                cy,
            } = arg;

            Self {
                tagsize,
                fx,
                fy,
                cx,
                cy,
            }
        }
    }

    impl FromStr for TagParamsArg {
        type Err = Error;

        fn from_str(text: &str) -> Result<Self, Self::Err> {
            let tokens: Vec<_> = text.split(',').collect();
            ensure!(
                tokens.len() == 5,
                r#"tag parameters must be in format "tagsize,fx,fy,cx,cy""#
            );

            let values = tokens
                .into_iter()
                .map(|token| -> Result<_> {
                    let value: f64 = token.parse()?;
                    Ok(value)
                })
                .collect::<Result<Vec<_>>>()
                .with_context(|| format!("failed to parse tag parameters {}", text))?;

            Ok(Self {
                tagsize: values[0],
                fx: values[1],
                fy: values[2],
                cx: values[3],
                cy: values[4],
            })
        }
    }

    pub fn _main() -> Result<()> {
        let Args {
            family: family_name,
            tag_params,
            input_files,
        } = argh::from_env();

        if input_files.is_empty() {
            eprintln!("no input files");
            return Ok(());
        }

        let family: Family = family_name.parse()?;
        let tag_params: Option<TagParams> = tag_params.map(|params| params.into());
        let mut detector = DetectorBuilder::new()
            .add_family_bits(family, 1)
            .build()
            .unwrap();

        for path in input_files.into_iter() {
            let image = image::open(&path)?;
            let detections = detector.detect(image.to_luma());

            println!("= image {}", path.display());

            detections.into_iter().enumerate().for_each(|(index, det)| {
                println!("  - detection {}: {:#?}", index, det);
                if let Some(tag_params) = &tag_params {
                    let pose = det.estimate_tag_pose(tag_params);
                    println!("  - pose {}: {:#?}", index, pose);

                    let isometry = pose.map(|pose| pose.to_isometry());
                    println!("  - isometry {}: {:#?}", index, isometry);
                }
            });
        }
        Ok(())
    }
}

#[cfg(feature = "full")]
fn main() -> Result<()> {
    simple_detector::_main()
}

#[cfg(not(feature = "full"))]
fn main() -> Result<()> {
    panic!(r#"please enable the "full" feature to run the example"#);
}
