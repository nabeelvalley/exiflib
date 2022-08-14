use exiflib::exif;
use exiflib::exif::TagFormat;
use std::env;
use std::fs;

fn parse_path(args: Vec<String>) -> String {
    let resolved = args.get(1);

    resolved.expect("path was not provided").to_string()
}

fn process_jpeg(file: &[u8]) {
    let exif = exif::parse(file).expect("Exif could not be parsed");

    println!("entries:");
    exif.get_entries().iter().for_each(|(tag_type, entry)| {
        if let TagFormat::AsciiString = entry.format {
            println!("{:?} 0x{:x} {:?}", tag_type, entry.tag, entry.value)
        }
    });
}

fn main() {
    let resolved = parse_path(env::args().collect());

    let file = fs::read(&resolved).unwrap();

    process_jpeg(&file);
}
