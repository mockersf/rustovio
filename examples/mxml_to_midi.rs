use std::env;

use rustovio::VerovioToolkit;

fn main() {
    let filename = match env::args().nth(1) {
        None => panic!("please provide a MusicXML file"),
        Some(file) => {
            eprintln!("reading {}", file);
            file
        }
    };

    let to = match env::args().nth(2) {
        None => panic!("please provide an output .mid file"),
        Some(file) => {
            eprintln!("writing to {}", file);
            file
        }
    };

    let mut tk = VerovioToolkit::new("verovio/data").unwrap();
    tk.load_data_from_file(filename);
    tk.render_to_midi_writer(std::fs::File::create(to).unwrap());
}
