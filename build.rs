use std::fs;
use std::path::Path;

fn main() {
    fs::create_dir_all(
        Path::new("target")
        .join("debug")
        .join("resources"))
        .expect("Failed to create resource forlder for debug");
    
        fs::create_dir_all(
            Path::new("target")
                    .join("release")
                    .join("resources"))
                        .expect_err("Failed to create resource folder for release");

    fs::copy(
        Path::new("resources")
                .join("config.yaml"), 
                Path::new("target")
                    .join("debug")
                    .join("resources")
                    .join("config.yaml")).expect("Failed to copy config.yaml to resources folder in debug");
    
    fs::copy(
        Path::new("resources")
            .join("config.yaml"), 
            Path::new("target")
                .join("release")
                .join("resources")
                .join("config.yaml")).expect("Failed to copy config.yaml to resources folder in release");
}