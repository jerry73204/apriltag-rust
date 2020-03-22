# apriltag-sys

[![Crates.io](https://img.shields.io/crates/v/apriltag-sys.svg)](https://crates.io/crates/apriltag-sys)
[![Documentation](https://docs.rs/apriltag-sys/badge.svg)](https://docs.rs/apriltag-sys/)
[![Crate License](https://img.shields.io/crates/l/apriltag-sys.svg)](https://crates.io/crates/apriltag-sys)
[![Dependency status](https://deps.rs/repo/github/jerry73204/apriltag-sys/status.svg)](https://deps.rs/repo/github/jerry73204/apriltag-sys)
[![build](https://github.com/jerry73204/apriltag-sys/workflows/build/badge.svg?branch=master)](https://github.com/jerry73204/apriltag-sys/actions?query=branch%3Amaster)

This crate provides Rust bindings for AprilTag C library.

## Usage

Install AprilTag library from official [repository](https://github.com/AprilRobotics/apriltag).

Import `apriltag-sys` dependency in your `Cargo.toml`

```toml
[dependencies]
apriltag-sys = "^0.1.2"
```

### Specifying how to compile and link the apriltag C library.

There are currently four options to specify how apriltag-sys will compile and
link the apriltag C library. These are specified by setting the
`APRILTAG_SYS_METHOD` environment variable to one of the following values:

- `pkg-config-then-static` (default) - This will try pkg-config first, then
   will fallback to `raw,static`.
- `pkg-config` - This will use pkg-config. Panic upon failure.
- `raw,static` - The environment variable `APRILTAG_SRC` must be set to a
  directory with the April Tag C library source code. The .c files will be
  compiled by directly calling the C compiler and statically linked.
- `cmake,dynamic` - The environment variable `APRILTAG_SRC` must be set to a
  directory with the April Tag C library source code. The cmake command will be
  invoked to call the C compiler and the resulting library will be dynamically
  linked.

The location of the apriltag source is specified by the `APRILTAG_SRC`
environment variable. If this is not set, a local git submodule checkout of the
apriltag source will be used.

## License

BSD-2-Clause. Please see [license file](LICENSE).
