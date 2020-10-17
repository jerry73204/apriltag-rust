# apriltag-rust

High-level API for AprilTag library built on top of [apriltag-sys](https://crates.io/crates/apriltag-sys).

## Usage

### Import to your project

Follow the install instructions on official [repository](https://github.com/AprilRobotics/apriltag)
to install AprilTag library.

Import apriltag-sys dependency in your Cargo.toml

```toml
[dependencies]
apriltag = "0.3"
```

### Feature Flags

The feature flags control the supported conversions from/to third-party types. It includes

- **full**: Enable most available features.
- **nalgebra**: Get type conversions from/to two dimensional byte matrix from [nalgebra](https://crates.io/crates/nalgebra) crate.
- **image**: Get type conversions from/to image types from [image](https://crates.io/crates/image) crate.

### Customize the build

If you would like to customize the way to link the AprilTag library,
please read the notes in [apriltag-sys](https://crates.io/crates/apriltag-sys) README.

## Example

To run detection on an image, run

```sh
cargo run --features full --example detector -- input.jpg
```

It accepts additional arguments:

- `--family tag36h11` specifies the tag36h11 tag family
- `--tag-params 1,2.1,2.2,4,5` sets the tag size, fx, fy, cx and cy parameters. It enable pose estimation feature.

```sh
cargo run --features full --example detector -- \
    --family tag36h11 \
    --tag-params 1,2.1,2.2,4,5 \
    input.jpg
```

The demo implementation can be found in [examples](examples) directory.

## License

BSD 2-Clause License. See [LICENSE file](LICENSE).
