use std::env;
use std::path::PathBuf;

use cc;
use bindgen;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cpp/adder.cpp")
        .include("cpp")
        .compile("adder");

    let bindings = bindgen::Builder::default()
        .clang_arg("-Icpp")
        .clang_args(&["-x", "c++"])
        .clang_arg("-std=c++14")
        .header("cpp/adder.hpp")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
