# apriltag-rust

High-level API for AprilTag library built on top of [apriltag-sys](https://crates.io/crates/apriltag-sys).

## Usage

### Import to your project

Follow the install instructions on official [repository](https://github.com/AprilRobotics/apriltag)
to install AprilTag library.

Import apriltag-sys dependency in your Cargo.toml

```toml
[dependencies]
apriltag = "0.1.0"
```

### Feature Flags

The feature flags control the supported conversions from/to third-party types. It includes

- **nalgebra**: Get type conversions from/to two dimensional byte matrix from [nalgebra](https://crates.io/crates/nalgebra) crate.
- **image**: Get type conversions from/to image types from [image](https://crates.io/crates/image) crate.

### Customize the build

If you would like to customize the way to compile the AprilTag library,
please read the notes in [apriltag-sys](https://crates.io/crates/apriltag-sys) README.

## Example

The snipplet works with [image](https://crates.io/crates/image) crate and prints marker detections.
You may check the [example](example) directory for complete runnable examples.


```rust
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
```

## License

BSD 2-Clause License. See [LICENSE file](LICENSE).
