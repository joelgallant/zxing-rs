extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    let context = cmake::Config::new("vendor").build_target("libzxing").build();

    println!("cargo:rustc-link-lib=static={}", "zxing");
    println!("cargo:rustc-link-search=native={}", context.join("build").display());

    bindgen::builder()
        .header("src/wrapper.hpp")
        .clang_arg("-Ivendor/core/src")
        .clang_arg("-std=c++14")
        .enable_cxx_namespaces()
        .opaque_type("std.*")
        .whitelist_type("zxing::.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
