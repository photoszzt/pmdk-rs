extern crate bindgen;
extern crate pmdk_builder;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = generate_bindings();

    bindings
        .write_to_file(PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write pmemobj bindings!");
}

#[cfg(not(target_os = "windows"))]
fn generate_bindings() -> bindgen::Bindings {
    pmdk_builder::build_lib("libpmemobj");
    let lib_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("build");
    println!("cargo:rustc-link-search={}", lib_path.as_path().display());

    bindgen::Builder::default()
        .header("libpmemobj.h")
        .blacklist_type("max_align_t")
        .derive_default(true)
        .derive_hash(true)
        .derive_debug(true)
        .generate()
        .expect("Unable to generate bindings")
}

#[cfg(target_os = "windows")]
fn generate_bindings() -> bindgen::Bindings {
    let lib = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("vcpkg/packages/pmdk_x64-windows/lib");
    println!("cargo:rustc-link-search={}", lib.as_path().display());
    bindgen::Builder::default()
        .header("libpmemobj.h")
        .header("pmemcompat.h")
        .blacklist_type("max_align_t")
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings")
}
