use std::env;
use std::path::PathBuf;

fn main() {
    // Build verovio. Enable the WASM build as this will also build the c static library
    let dst = cmake::Config::new("verovio/cmake")
        .define("BUILD_AS_WASM", "ON")
        .build();
    println!("cargo:rustc-link-search={}/build", dst.display());

    // Static link to verovio
    println!("cargo:rustc-link-lib=static=verovio");

    // Dynamic link to cpp runtime
    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else {
        unimplemented!();
    }

    println!("cargo:rerun-if-changed=verovio/tools/c_wrapper.h");

    // Generate the bindings from C api
    let bindings = bindgen::Builder::default()
        .header("verovio/tools/c_wrapper.h")
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        .clang_arg("-fno-threadsafe-statics")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
