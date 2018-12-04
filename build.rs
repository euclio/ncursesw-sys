extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let lib_name = if cfg!(target_os = "macos") {
        "ncurses"
    } else {
        "ncursesw"
    };

    println!("cargo:rustc-link-lib={}", lib_name);

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
