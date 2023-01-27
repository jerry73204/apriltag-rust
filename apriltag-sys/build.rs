use lazy_static::lazy_static;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref MANIFEST_DIR: PathBuf = {
        PathBuf::from(
            std::env::var_os("CARGO_MANIFEST_DIR")
                .expect("The CARGO_MANIFEST_DIR environment variable is not set"),
        )
    };
    static ref OUT_DIR: PathBuf = {
        PathBuf::from(
            std::env::var_os("OUT_DIR").expect("The OUT_DIR environment variable is not set"),
        )
    };
}

enum SrcMethod {
    Cmake(PathBuf),
    RawStatic(PathBuf),
    PkgConfig,
    PkgConfigThenStatic(PathBuf),
}

fn get_source_method() -> SrcMethod {
    let src = std::env::var_os("APRILTAG_SRC")
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from("apriltag-src")); // git submodule checks this out
    let method: String = std::env::var_os("APRILTAG_SYS_METHOD")
        .map(|s| {
            s.into_string()
                .expect("If set, APRILTAG_SYS_METHOD environment variable must be UTF-8 string.")
        })
        .unwrap_or("pkg-config-then-static".to_string()); // This is the default

    match method.as_str() {
        "pkg-config" => SrcMethod::PkgConfig,
        "pkg-config-then-static" => SrcMethod::PkgConfigThenStatic(src),
        "raw,static" => SrcMethod::RawStatic(src),
        "cmake,dynamic" => SrcMethod::Cmake(src),
        _ => {
            panic!(
                "The APRILTAG_SYS_METHOD value \"{}\" was not recognized. See README.md of the \
                 apriltag-sys crate for a description of this environment variable.",
                method
            );
        }
    }
}

#[derive(Debug)]
struct Error {}

impl From<glob::PatternError> for Error {
    fn from(_: glob::PatternError) -> Error {
        Error {}
    }
}

impl From<glob::GlobError> for Error {
    fn from(_: glob::GlobError) -> Error {
        Error {}
    }
}

fn main() -> Result<(), Error> {
    use SrcMethod::*;

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=APRILTAG_SRC");
    println!("cargo:rerun-if-env-changed=APRILTAG_SYS_METHOD");
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rerun-if-env-changed=APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR");
        println!("cargo:rerun-if-env-changed=APRILTAG_SYS_WINDOWS_PTHREAD_STATIC_LIB");
        println!("cargo:rerun-if-env-changed=APRILTAG_SYS_WINDOWS_NO_WINMM");
    }

    // Detect which method to use.
    #[allow(unused_variables)]
    let clang_args = {
        let mut args = match get_source_method() {
            Cmake(src_path) => build_cmake(src_path),
            RawStatic(sdk_path) => build_raw_static(sdk_path)?,
            other => {
                // check if apriltag is available on system
                match pkg_config::probe_library("apriltag") {
                    Ok(_) => vec![],
                    Err(e) => {
                        if let PkgConfigThenStatic(sdk_path) = other {
                            build_raw_static(sdk_path)?
                        } else {
                            panic!("pkg-config failed: {}", e);
                        }
                    }
                }
            }
        };

        // tell clang keep the comments due to the issue
        // https://github.com/rust-lang/rust-bindgen/issues/426
        args.push("-fretain-comments-from-system-headers".into());

        // Allow bindgen to find a pthreads shim on Microsoft Windows
        #[cfg(target_os = "windows")]
        {
            if let Some(path) = std::env::var_os("APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR")
            .map(|s| {
                std::path::PathBuf::from(s.into_string()
                    .expect("If set, APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR environment variable must be UTF-8 string."))
            }) {
                args.push(format!("-I{}", path.display()));
            }
        }

        args
    };

    // If we need to regenerate the .rs files for bindings, do that, too.
    #[cfg(feature = "buildtime-bindgen")]
    {
        let bindgen_builder = bindgen::Builder::default()
            .header("wrapper.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate_comments(true)
            .allowlist_type("apriltag_.*")
            .allowlist_type("image_u8_.*")
            .allowlist_type("image_u8x3_.*")
            .allowlist_type("image_u8x4_.*")
            .allowlist_type("zarray_.*")
            .allowlist_type("matd_.*")
            .allowlist_function("apriltag_.*")
            .allowlist_function("estimate_.*")
            .allowlist_function("tag16h5_.*")
            .allowlist_function("tag25h9_.*")
            .allowlist_function("tag36h11_.*")
            .allowlist_function("tagCircle21h7_.*")
            .allowlist_function("tagCircle49h12_.*")
            .allowlist_function("tagCustom48h12_.*")
            .allowlist_function("tagStandard41h12_.*")
            .allowlist_function("tagStandard52h13_.*")
            .allowlist_function("image_u8_.*")
            .allowlist_function("image_u8x3_.*")
            .allowlist_function("image_u8x4_.*")
            .allowlist_function("zarray_.*")
            .allowlist_function("matd_.*");
        let bindgen_builder = bindgen_builder.clang_args(clang_args);
        let bindings = bindgen_builder
            .generate()
            .expect("Unable to generate bindings");

        let bindings_path = MANIFEST_DIR.join("bindings").join("bindings.rs");
        std::fs::create_dir_all(bindings_path.parent().unwrap()).unwrap();
        bindings
            .write_to_file(bindings_path)
            .expect("Couldn't write bindings!");
    }

    // On Microsoft Windows, apriltag requires an additional dependency as pthread is not available by default. Add a required shim as static library.
    #[cfg(target_os = "windows")]
    {
        match std::env::var_os("APRILTAG_SYS_WINDOWS_PTHREAD_STATIC_LIB")
        .map(|s| {
            std::path::PathBuf::from(s.into_string()
                .expect("If set, APRILTAG_SYS_WINDOWS_PTHREAD_STATIC_LIB environment variable must be UTF-8 string."))
        }) {
            Some(pthread_static_lib) if pthread_static_lib.is_file() => {
                // Add path if the file is not directly placed in a root directory.
                if let Some(pthread_static_lib_dir) = pthread_static_lib.parent() {
                    println!("cargo:rustc-link-search={}", pthread_static_lib_dir.display());
                }
                println!("cargo:rustc-link-lib={}", pthread_static_lib.file_stem().expect("Valid file").to_str().expect("Valid UTF-8"));

                // Currently, some shims require the function "gettimeofday" not available by default. Linking to winmm fix this issue.
                if std::env::var_os("APRILTAG_SYS_WINDOWS_NO_WINMM").is_none() {
                    println!("cargo:rustc-link-lib=winmm");
                }
            }
            Some(pthread_static_lib) => {
                println!("cargo:warning=The given path for the static library of pthread '{}' specified by APRILTAG_SYS_WINDOWS_PTHREAD_STATIC_LIB is not a valid file", pthread_static_lib.display());
                std::process::exit(2)
            }
            None => {
                println!("cargo:warning=Under Microsoft Windows, apriltags' dependency 'pthread' is not available by default. Consequently, link.exe is likely to fail with LNK2019 in the next step. Consider installing a shim and specify APRILTAG_SYS_WINDOWS_PTHREAD_STATIC_LIB as path to a static library like 'pthreadVC3.lib'.");
            }
        }
    }

    Ok(())
}

/// Use cmake to build April Tags as a shared library.
fn build_cmake<P>(src_path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    println!("cargo:rustc-link-lib=apriltag");

    let build_dir = OUT_DIR.join("cmake_build");
    std::fs::create_dir_all(&build_dir).unwrap();

    let dst = cmake::Config::new(src_path).out_dir(build_dir).build();

    let include_dir = dst.join("include");
    let lib_dir = dst.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    vec![
        format!("-I{}", include_dir.display()),
        format!("-L{}", lib_dir.display()),
    ]
}

/// Build April Tags source by passing all .c files and compiling static lib.
fn build_raw_static(sdk_path: PathBuf) -> Result<Vec<String>, Error> {
    let inc_dir = sdk_path.clone();

    let mut compiler = cc::Build::new();

    // add files in base SDK dir
    let mut c_files = sdk_path.clone();
    c_files.push("*.c");
    let glob_pattern = c_files.to_str().unwrap();
    let paths = glob::glob_with(glob_pattern, glob::MatchOptions::new())?;
    let mut count = 0;
    for path in paths {
        let path = path?;
        let path_str = path.display().to_string();
        if path_str.ends_with("apriltag_pywrap.c") {
            continue;
        }
        compiler.file(&path);
        count += 1;
    }
    if count == 0 {
        panic!(
            "No source files found at {}. Hint: do 'git submodule update --init'.",
            glob_pattern
        );
    }

    // add files in base/common SDK dir
    let mut c_files = sdk_path.clone();
    c_files.push("common");
    c_files.push("*.c");
    let glob_pattern = c_files.to_str().unwrap();
    let paths = glob::glob_with(glob_pattern, glob::MatchOptions::new())?;
    for path in paths {
        compiler.file(&path?);
    }

    compiler.include(&inc_dir);
    compiler.extra_warnings(false);

    // On Microsoft Windows, apriltag requires an additional dependency as PTHREAD is not available by default.
    #[cfg(target_os = "windows")]
    {
        match std::env::var_os("APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR")
        .map(|s| {
            std::path::PathBuf::from(s.into_string()
                .expect("If set, APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR environment variable must be UTF-8 string."))
        }) {
            Some(pthread_include_dir) if pthread_include_dir.is_dir() => {
                compiler.include(pthread_include_dir);
            }
            Some(pthread_include_dir) => {
                println!("cargo:warning=The given include directory for pthread '{}' specified by APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR is not a valid directory in the file system.", pthread_include_dir.display());
                std::process::exit(2)
            }
            None => {
                println!("cargo:warning=Under Microsoft Windows, apriltags' dependency 'pthread' is not available by default. Consequently, cl.exe is likely to fail in the next step. Consider installing a shim and specify APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR accordingly.");
            }
        }
    }

    compiler.compile("apriltags");

    Ok(vec![])
}
