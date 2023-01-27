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
apriltag-sys = "0.2"
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

#### Building under Windows

Strictly speaking, using apriltag on Microsoft Windows is not *officially* supported by the developers. In practice, the library works well even on this operating system. 
The only additional complexity emerges during the building process. The C library requires *pthread.h* not shipped with Windows by default.
Consequently, different shims like [pthreads4w](https://sourceforge.net/projects/pthreads4w/) and [Pthreads-w32](https://www.sourceware.org/pthreads-win32/) are required.
If one of them is installed, setting the environment variables `APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR` to its include directory and `APRILTAG_SYS_WINDOWS_PTHREAD_STATIC_LIB` to the compiled static library allows a successful build under Windows with `APRILTAG_SYS_METHOD=raw,static`.

As an example using [vcpkg](https://github.com/microsoft/vcpkg), building under Windows consists of three additional steps:
1. Install the shim using `vcpkg install pthread:x64-windows-static`
2. Specify the include directoy (here in PowerShell): `$env:APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR="%SPECIFY VCPKG DIRECTORY HERE%\installed\x64-windows-static\include"`
3. Specify the path to the static library (again in PowerShell): `$env:APRILTAG_SYS_WINDOWS_PTHREAD_STATIC_LIB="%SPECIFY VCPKG DIRECTORY HERE%\installed\x64-windows-static\lib\pthreadVC3.lib""`

Some shims require `winmm.dll` for high-precision timing shipped by default with all Windows installations. If this linking is not necessary, it can be omitted by setting `APRILTAG_SYS_WINDOWS_NO_WINMM=1`.

## License

BSD-2-Clause. Please see the [license file](LICENSE).
