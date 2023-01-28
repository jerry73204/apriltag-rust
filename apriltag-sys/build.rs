use anyhow::{anyhow, bail, ensure, Context, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{
    env::{self, VarError},
    fs,
    path::{Path, PathBuf},
};

static MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
static OUT_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let path = env::var_os("OUT_DIR").expect("The OUT_DIR environment variable is not set");
    PathBuf::from(path)
});

enum SrcMethod {
    Cmake(PathBuf),
    RawStatic(PathBuf),
    PkgConfig,
    PkgConfigThenStatic(PathBuf),
}

fn get_source_method() -> Result<SrcMethod> {
    use SrcMethod as M;

    let src = env::var_os("APRILTAG_SRC")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("apriltag-src")); // git submodule checks this out

    let value = match env::var("APRILTAG_SYS_METHOD") {
        Ok(value) => value,
        Err(VarError::NotUnicode(_)) => {
            bail!("If set, APRILTAG_SYS_METHOD environment variable must be UTF-8 string.")
        }
        Err(VarError::NotPresent) => return Ok(M::PkgConfigThenStatic(src)), // This is default
    };

    let method = match value.as_str() {
        "pkg-config" => M::PkgConfig,
        "pkg-config-then-static" => M::PkgConfigThenStatic(src),
        "raw,static" => M::RawStatic(src),
        "cmake,dynamic" => M::Cmake(src),
        value => {
            bail!(
                r#"The APRILTAG_SYS_METHOD value "{value}" was not recognized. See README.md of the \
                 apriltag-sys crate for a description of this environment variable."#,
            );
        }
    };

    Ok(method)
}

fn main() -> Result<()> {
    use SrcMethod as M;

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
        let mut args = match get_source_method()? {
            M::Cmake(src_path) => build_cmake(src_path)?,
            M::RawStatic(sdk_path) => build_raw_static(sdk_path)?,
            other => {
                // check if apriltag is available on system
                match pkg_config::probe_library("apriltag") {
                    Ok(_) => vec![],
                    Err(e) => {
                        if let M::PkgConfigThenStatic(sdk_path) = other {
                            build_raw_static(sdk_path)?
                        } else {
                            bail!("pkg-config failed: {e}");
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
            let path = env::var_os("APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR").map(PathBuf::from);
            if let Some(path) = path {
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
            .context("Unable to generate bindings")?;

        let bindings_path = Path::new(MANIFEST_DIR).join("bindings").join("bindings.rs");
        fs::create_dir_all(bindings_path.parent().unwrap())?;
        bindings
            .write_to_file(bindings_path)
            .context("Couldn't write bindings!")?;
    }

    // On Microsoft Windows, apriltag requires an additional dependency as pthread is not available by default. Add a required shim as static library.
    #[cfg(target_os = "windows")]
    {
        match env::var_os("APRILTAG_SYS_WINDOWS_PTHREAD_STATIC_LIB").map(PathBuf::from) {
            Some(pthread_static_lib) if pthread_static_lib.is_file() => {
                // Add path if the file is not directly placed in a root directory.
                if let Some(pthread_static_lib_dir) = pthread_static_lib.parent() {
                    println!(
                        "cargo:rustc-link-search={}",
                        pthread_static_lib_dir.display()
                    );
                }
                let lib_name = pthread_static_lib
                    .file_stem()
                    .ok_or_else(|| anyhow!("file_stem() returns None"))?
                    .to_str()
                    .ok_or_else(|| anyhow!("to_str() returns None"))?;
                println!("cargo:rustc-link-lib={}", lib_name);

                // Currently, some shims require the function "gettimeofday" not available by default. Linking to winmm fix this issue.
                if env::var_os("APRILTAG_SYS_WINDOWS_NO_WINMM").is_none() {
                    println!("cargo:rustc-link-lib=winmm");
                }
            }
            Some(pthread_static_lib) => {
                bail!("cargo:warning=The given path for the static library of pthread '{}' specified by APRILTAG_SYS_WINDOWS_PTHREAD_STATIC_LIB is not a valid file", pthread_static_lib.display());
            }
            None => {
                println!("cargo:warning=Under Microsoft Windows, apriltags' dependency 'pthread' is not available by default. Consequently, link.exe is likely to fail with LNK2019 in the next step. Consider installing a shim and specify APRILTAG_SYS_WINDOWS_PTHREAD_STATIC_LIB as path to a static library like 'pthreadVC3.lib'.");
            }
        }
    }

    Ok(())
}

/// Use cmake to build April Tags as a shared library.
fn build_cmake<P>(src_path: P) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    println!("cargo:rustc-link-lib=apriltag");

    let build_dir = OUT_DIR.join("cmake_build");
    fs::create_dir_all(&build_dir)?;

    let dst = cmake::Config::new(src_path).out_dir(build_dir).build();

    let include_dir = dst.join("include");
    let lib_dir = dst.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    Ok(vec![
        format!("-I{}", include_dir.display()),
        format!("-L{}", lib_dir.display()),
    ])
}

/// Build April Tags source by passing all .c files and compiling static lib.
fn build_raw_static(sdk_path: PathBuf) -> Result<Vec<String>> {
    let inc_dir = &sdk_path;

    let mut compiler = cc::Build::new();

    // add files in base SDK dir and base/common SDK dir
    let src_dirs = [&sdk_path, &sdk_path.join("common")];
    let count = src_dirs
        .into_iter()
        .map(fs::read_dir)
        .flatten_ok()
        .map(|entry| anyhow::Ok(entry??.path()))
        .filter_ok(|path| matches!(path.extension(), Some(ext) if ext == "c"))
        .filter_ok(|path| !path.ends_with("apriltag_pywrap.c"))
        .try_fold(0, |count, path| -> Result<_> {
            compiler.file(path?);
            Ok(count + 1)
        })?;

    ensure!(
        count > 0,
        "No source files found at {}. Hint: do 'git submodule update --init'.",
        sdk_path.display()
    );

    compiler.include(inc_dir);
    compiler.extra_warnings(false);

    // On Microsoft Windows, apriltag requires an additional dependency as PTHREAD is not available by default.
    #[cfg(target_os = "windows")]
    {
        match env::var_os("APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR").map(PathBuf::from) {
            Some(pthread_include_dir) if pthread_include_dir.is_dir() => {
                compiler.include(pthread_include_dir);
            }
            Some(pthread_include_dir) => {
                bail!("The given include directory for pthread '{}' specified by APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR is not a valid directory in the file system.", pthread_include_dir.display());
            }
            None => {
                println!("cargo:warning=Under Microsoft Windows, apriltags' dependency 'pthread' is not available by default. Consequently, cl.exe is likely to fail in the next step. Consider installing a shim and specify APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR accordingly.");
            }
        }
    }

    compiler.compile("apriltags");

    Ok(vec![])
}
