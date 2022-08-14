use rawlib::exif;
use rawlib::fuji;
use std::env;
use std::fs;

fn parse_path(args: Vec<String>) -> String {
    let resolved = args.get(1);

    resolved.expect("path was not provided").to_string()
}

fn jpeg_only(args: Vec<String>) -> bool {
    let resolved = args.get(2);

    matches!(resolved, Some(_))
}

fn process_raw(file: Vec<u8>, resolved: String) {
    let result = crate::fuji::parse(&file).expect("result does not exist");

    println!(
        "{} {} {} {}",
        result.format, result.identifier, result.model, result.version
    );

    let jpeg_path = format!("{}.jpeg", &resolved);

    fs::write(&jpeg_path, &result.jpeg.bytes).expect("failed to write jpeg");

    println!("jpeg written to {}", &jpeg_path);
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
    let is_jpeg = jpeg_only(env::args().collect());

    let file = fs::read(&resolved).unwrap();

    if is_jpeg {
        process_jpeg(&file);
    } else {
        process_raw(file, resolved);
    }
}
