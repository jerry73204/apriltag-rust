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
            std::env::var_os("OUT_DIR")
                .expect("The CARGO_MANIFEST_DIR environment variable is not set"),
        )
    };
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=APRILTAG_SRC");

    #[cfg(feature = "buildtime-bindgen")]
    {
        let clang_args = match std::env::var_os("APRILTAG_SRC") {
            Some(src_path) => {
                // build apriltag from source
                let dst = build_source(src_path);
                let include_dir = dst.join("include");
                let lib_dir = dst.join("lib");
                println!("cargo:rustc-link-search=native={}", lib_dir.display());

                vec![
                    format!("-I{}", include_dir.display()),
                    format!("-L{}", lib_dir.display()),
                ]
            }
            None => {
                // check if apriltag is available on system
                let _ = pkg_config::probe_library("apriltag").unwrap();
                vec![]
            }
        };

        let builder = bindgen::Builder::default()
            .header("wrapper.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .whitelist_type("apriltag_.*")
            .whitelist_type("image_u8_.*")
            .whitelist_type("image_u8x3_.*")
            .whitelist_type("image_u8x4_.*")
            .whitelist_type("zarray_.*")
            .whitelist_type("matd_.*")
            .whitelist_function("apriltag_.*")
            .whitelist_function("tag16h5_.*")
            .whitelist_function("tag25h9_.*")
            .whitelist_function("tag36h11_.*")
            .whitelist_function("tagCircle21h7_.*")
            .whitelist_function("tagCircle49h12_.*")
            .whitelist_function("tagCustom48h12_.*")
            .whitelist_function("tagStandard41h12_.*")
            .whitelist_function("tagStandard52h13_.*")
            .whitelist_function("image_u8_.*")
            .whitelist_function("image_u8x3_.*")
            .whitelist_function("image_u8x4_.*")
            .whitelist_function("zarray_.*")
            .whitelist_function("matd_.*");
        let builder = builder.clang_args(clang_args);
        let bindings = builder.generate().expect("Unable to generate bindings");

        let bindings_path = MANIFEST_DIR.join("bindings").join("bindings.rs");
        std::fs::create_dir_all(bindings_path.parent().unwrap()).unwrap();
        bindings
            .write_to_file(bindings_path)
            .expect("Couldn't write bindings!");
    }

    println!("cargo:rustc-link-lib=apriltag");
}

#[cfg(feature = "buildtime-bindgen")]
fn build_source<P>(src_path: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let build_dir = OUT_DIR.join("cmake_build");
    std::fs::create_dir_all(&build_dir).unwrap();

    let dst = cmake::Config::new(src_path).out_dir(build_dir).build();
    dst
}
