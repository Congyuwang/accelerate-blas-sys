extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=framework=Accelerate");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let sdk = std::process::Command::new("xcrun")
        .args("--sdk macosx --show-sdk-path".split(" "))
        .output()
        .expect("failed to execute xcrun");

    let sdk_root = String::from_utf8_lossy(&sdk.stdout);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .generate_comments(true)
        .generate_inline_functions(true)
        // The input header we would like to generate
        // bindings for.
        .header("./headers/blas.h")
        // add xcode as clang sys-root
        .clang_arg(format!("-isysroot{}", sdk_root.trim()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
