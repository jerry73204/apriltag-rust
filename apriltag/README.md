# apriltag crate

High-level API for AprilTag library built on top of
[apriltag-sys](https://crates.io/crates/apriltag-sys).

## Usage

### Import to your project

Add apriltag crate to your `Cargo.toml`.

```sh
cargo add apriltag
```

### Customize the build (optional)

The apriltag crate ships and statically links the AprilTag C library
by default. If you would like to customize the way to link the
AprilTag library, please read the notes in
[apriltag-sys](https://crates.io/crates/apriltag-sys) README.

## Example

To run apriltag detection on an PNM image,

```sh
cargo run --example detector -- input.pnm
```

It accepts additional arguments:

```sh
cargo run --example detector -- \
    --family tag36h11 \
    --tag-params 1,2.1,2.2,4,5 \
    input.pnm
```

where the arguments are explained as follows.

- `--family tag36h11` specifies the tag36h11 tag family.
- `--tag-params 1,2.1,2.2,4,5` sets the tag size, fx, fy, cx and cy parameters. It enable pose estimation feature.


## Third-party type conversions

Third-party type conversions are supported by extension crates, including

- [apriltag-nalgebra](https://crates.io/crates/apriltag-nalgebra): Add
  conversions from/to two dimensional byte matrix in nalgebra crate.
- [apriltag-image](https://crates.io/crates/apriltag-image): Add
  conversions from/to image types in image crate.


## License

BSD 2-Clause License. See [LICENSE file](LICENSE).
