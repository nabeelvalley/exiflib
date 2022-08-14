use exiflib::exif;
use std::env;
use std::fs;

fn parse_path(args: Vec<String>) -> String {
    let resolved = args.get(1);

    resolved.expect("path was not provided").to_string()
}

fn process_jpeg(file: &[u8]) {
    let exif = exif::parse(file).expect("Exif could not be parsed");

    println!("entries");

    if let Some(entries) = exif.get_entries() {
        entries.iter().for_each(|entry| {
            println!("0x{:x} {:?}", entry.tag, entry.value);
        });
    }
}

fn main() {
    let resolved = parse_path(env::args().collect());

    let file = fs::read(&resolved).unwrap();

    process_jpeg(&file);
}
