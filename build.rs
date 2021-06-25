use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::Config::new("verovio/cmake")
        .define("BUILD_AS_WASM", "ON")
        .build();
    println!("cargo:rustc-link-search={}/build", dst.display());

    println!("cargo:rustc-link-lib=static=verovio");

    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else {
        unimplemented!();
    }

    println!("cargo:rerun-if-changed=verovio/tools/c_wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("verovio/tools/c_wrapper.h")
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        .clang_arg("-fno-threadsafe-statics")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
