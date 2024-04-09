use std::env;
use std::path::PathBuf;

fn main() {
    // Tell Cargo to tell rustc to link the system shared library.
    println!("cargo:rustc-link-lib=dylib=python3.8");

    // Path to the output directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Path to the generated lib.rs file
    let lib_path = out_dir.join("lib.rs");

    // Generate Python bindings using cbindgen
    let bindings = cbindgen::Config::from_file("cbindgen.toml")
        .unwrap()
        .generate()
        .unwrap();

    // Write the generated bindings to a file
    let bindings_path = out_dir.join("bindings.rs");
    bindings.write_to_file(&bindings_path).unwrap();

    // Print a message indicating the path to the generated bindings file
    println!("Generated bindings at {:?}", bindings_path);
}