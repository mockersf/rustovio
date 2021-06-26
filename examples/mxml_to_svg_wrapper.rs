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

    let mut tk = VerovioToolkit::new("verovio/data").unwrap();
    tk.load_data_from_file(filename);
    let svg_str = tk.render_to_svg(1);
    println!("{}", svg_str);
}
