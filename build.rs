// Example custom build script.
// use std::env;
use std::{fs, io};
use std::path::Path;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    // println!("cargo:rerun-if-changed=src/hello.c");

    // let target = env::var("OUT_DIR").unwrap(); //OUT_DIR

    // println!("{:?}", target);

    // fs::copy(Path::new("res").join("config.yaml"), Path::new(&target).join("res").join("config.yaml"));


    fs::create_dir_all(Path::new("target").join("debug").join("resources")).expect("Failed to create resource forlder for debug");
    fs::create_dir_all(Path::new("target").join("release").join("resources")).expect_err("Failed to create resource forlder for release");

    fs::copy(Path::new("resources").join("config.yaml"), Path::new("target").join("debug").join("resources").join("config.yaml")).expect("Failed to copy config.yaml to resources folder in debug");
    
    fs::copy(Path::new("resources").join("config.yaml"), Path::new("target").join("release").join("resources").join("config.yaml")).expect("Failed to copy config.yaml to resources folder in release");
    // Use the `cc` crate to build a C file and statically link it.
    // cc::Build::new()
    //     .file("rs/config.yaml")
        // .compile("hello");
}