use std::env;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs;

fn main() {
    let filename = match env::args().nth(1) {
        None => panic!("please provide a MusicXML file"),
        Some(file) => {
            println!("reading {}", file);
            file
        }
    };

    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data = CString::new(data).unwrap();

    let resource_folder = CString::new("data").unwrap();

    let tk =
        unsafe { rustovio::bindings::vrvToolkit_constructorResourcePath(resource_folder.as_ptr()) };
    dbg!(unsafe { rustovio::bindings::vrvToolkit_loadData(tk, data.as_ptr(),) });
    dbg!(unsafe { rustovio::bindings::vrvToolkit_renderToMIDI(tk, 0 as *const i8,) });
    let svg = dbg!(unsafe { rustovio::bindings::vrvToolkit_renderToSVG(tk, 1, 0 as *const i8) });
    let svg_str = unsafe { CStr::from_ptr(svg) };
    eprintln!("svg : '{}'", svg_str.to_str().unwrap());
}
