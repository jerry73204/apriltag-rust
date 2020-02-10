use std::path::PathBuf;

fn main() {
    // link apriltag library
    println!("cargo:rustc-link-lib=apriltag");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
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
        .whitelist_function("matd_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
