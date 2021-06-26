use std::{
    env,
    ffi::{CStr, CString},
    fs,
};

fn main() {
    let filename = match env::args().nth(1) {
        None => panic!("please provide a MusicXML file"),
        Some(file) => {
            eprintln!("reading {}", file);
            file
        }
    };

    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let cdata = CString::new(data).unwrap();

    let resource_folder = CString::new("verovio/data").unwrap();

    let svg_str = unsafe {
        let tk = rustovio::bindings::vrvToolkit_constructorResourcePath(resource_folder.as_ptr());
        rustovio::bindings::vrvToolkit_loadData(tk, cdata.as_ptr());
        let svg = rustovio::bindings::vrvToolkit_renderToSVG(tk, 1, std::ptr::null::<i8>());
        CStr::from_ptr(svg)
    };
    println!("{}", svg_str.to_str().unwrap());
}
