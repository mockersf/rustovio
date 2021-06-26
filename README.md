# Rustovio

[![CI](https://github.com/mockersf/rustovio/actions/workflows/ci.yml/badge.svg)](https://github.com/mockersf/rustovio/actions/workflows/ci.yml)
[![docs](https://docs.rs/rustovio/badge.svg)](https://docs.rs/rustovio)
[![crates.io](https://img.shields.io/crates/v/rustovio.svg)](https://crates.io/crates/rustovio)

Rust bindings to [Verovio](https://github.com/rism-digital/verovio). It offer the raw bindings and a limited wrapper.

## Limitations

This currrently does not work on Windows, mostly because I don't have one to check how to do the setup.

## Wrapper

```rust
let mut tk = VerovioToolkit::new("verovio/data").unwrap();
tk.load_data_from_file(filename);
println!("{}", tk.render_to_svg(1).unwrap());
```

## Bindings

```rust
let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
let cdata = CString::new(data).unwrap();

let resource_folder = CString::new("verovio/data").unwrap();

let svg_str = unsafe {
    let tk = bindings::vrvToolkit_constructorResourcePath(resource_folder.as_ptr());
    bindings::vrvToolkit_loadData(tk, cdata.as_ptr());
    let svg = bindings::vrvToolkit_renderToSVG(tk, 1, std::ptr::null::<i8>());
    CStr::from_ptr(svg)
};
println!("{}", svg_str.to_str().unwrap());
```
