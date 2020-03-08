# apriltag-sys

This crate provides Rust bindings for AprilTag C library.

## Usage

Install AprilTag library from official [repository](https://github.com/AprilRobotics/apriltag).

Import `apriltag-sys` dependency in your `Cargo.toml`

```toml
[dependencies]
apriltag-sys = "^0.1.2"
```

### Specifying how to compile and link the apriltag C library.

There are currently three options to specify how apriltag-sys will compile and
link the apriltag C library. These are specified by setting the
`APRILTAG_SYS_METHOD` environment variable to one of the following values:

- `pkg-config` (default) - This will use pkg-config.
- `raw,static` - The environment variable `APRILTAG_SRC` must be set to a
  directory with the April Tag C library source code. The .c files will be
  compiled by directly calling the C compiler and statically linked.
- `cmake,dynamic` - The environment variable `APRILTAG_SRC` must be set to a
  directory with the April Tag C library source code. The cmake command will be
  invoked to call the C compiler and the resulting library will be dynamically
  linked.

## License

BSD-2-Clause. Please see [license file](LICENSE).
