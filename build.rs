use std::env;

fn main() {
    println!("cargo:rustc-link-lib=PCANBasic"); // Link dynamically

    let target = env::var("TARGET").unwrap();

    // Ensure linking is handled correctly for Windows
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=PCANBasic"); // Dynamic linking
    }
}
