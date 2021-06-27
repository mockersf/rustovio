//! Bindings to Verovio

#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]

use std::{
    ffi::{c_void, CStr, CString},
    fs,
    path::PathBuf,
};

/// Generated Verovio bindings from the C API. Use at your own risk.
pub mod bindings {
    #![allow(missing_docs)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// Helper to work with the Verovio Toolkit.
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct VerovioToolkit {
    tk: *mut c_void,
}

impl VerovioToolkit {
    /// Create a new toolkit, using the data from the specified folder.
    pub fn new<P: Into<PathBuf>>(data_dir: P) -> Option<Self> {
        let resource_folder = CString::new(data_dir.into().to_str()?).unwrap();
        let tk = unsafe { bindings::vrvToolkit_constructorResourcePath(resource_folder.as_ptr()) };
        Some(Self { tk })
    }

    /// Load data from the sheet given.
    pub fn load_data(&mut self, data: &str) -> bool {
        let cdata = CString::new(data).unwrap();

        unsafe { bindings::vrvToolkit_loadData(self.tk, cdata.as_ptr()) }
    }

    /// Load data from the sheet file given.
    pub fn load_data_from_file<P: Into<PathBuf>>(&mut self, data_path: P) -> bool {
        if let Ok(data) = fs::read_to_string(data_path.into()) {
            self.load_data(&data)
        } else {
            false
        }
    }

    /// Render a sheet page as SVG.
    pub fn render_to_svg(&mut self, page_number: i32) -> Option<&str> {
        let svg_cstr = unsafe {
            let c_pointer =
                bindings::vrvToolkit_renderToSVG(self.tk, page_number, std::ptr::null::<i8>());
            CStr::from_ptr(c_pointer)
        };
        svg_cstr.to_str().ok()
    }

    /// Render a sheet as MIDI. This will return a base64 encoded string that is the content of a MIDI file.
    pub fn render_to_midi(&mut self) -> Option<&str> {
        let svg_cstr = unsafe {
            let c_pointer = bindings::vrvToolkit_renderToMIDI(self.tk, std::ptr::null::<i8>());
            CStr::from_ptr(c_pointer)
        };
        svg_cstr.to_str().ok()
    }

    /// Render a sheet as MIDI and write it to the given writer.
    #[cfg(feature = "midi-decoding")]
    pub fn render_to_midi_writer(&mut self, mut writer: impl std::io::Write) -> bool {
        self.render_to_midi()
            .and_then(|content| base64::decode(content.as_bytes()).ok())
            .and_then(|decoded| writer.write_all(&decoded).ok())
            .is_some()
    }
}
