use anyhow::{ensure, Context, Error, Result};
use apriltag::{Detector, Family, Image, TagParams};
use clap::Parser;
use std::str::FromStr;

/// Simple AprilTag detector.
#[derive(Debug, Clone, Parser)]
struct Opts {
    #[clap(long = "family", default_value = "tag16h5")]
    /// family name.
    pub family_name: String,

    #[clap(long)]
    /// optional tag parameters in format "tagsize,fx,fy,cx,cy".
    pub tag_params: Option<TagParamsArg>,

    /// a list of input PNM image files.
    pub input_files: Vec<String>,
}

fn main() -> Result<()> {
    let Opts {
        family_name,
        tag_params,
        input_files,
    } = Opts::parse();

    ensure!(!input_files.is_empty(), "no input files");

    let family: Family = family_name.parse()?;
    let tag_params: Option<TagParams> = tag_params.map(|params| params.into());
    let mut detector = Detector::builder().add_family_bits(family, 1).build()?;

    for path in input_files {
        let image = Image::from_pnm_file(&path)?;
        let detections = detector.detect(&image);

        println!("= image {path}");

        detections.into_iter().enumerate().for_each(|(index, det)| {
            println!("  - detection {index}: {det:#?}");
            if let Some(tag_params) = &tag_params {
                let pose = det.estimate_tag_pose(tag_params);
                println!("  - pose {index}: {pose:#?}");
                println!("  - isometry {index}: {pose:#?}");
            }
        });
    }
    Ok(())
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
