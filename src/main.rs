use rawlib::exif;
use rawlib::fuji;
use std::env;
use std::fmt::LowerHex;
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

fn process_jpeg(file: &[u8], resolved: String) {
    let exif = exif::parse(file).expect("Exif could not be parsed");

    println!("entries");

    if let Some(entries) = exif.get_entries() {
        for i in 0..entries.len() {
            println!("{:?}", entries[i]);
            let exif_value_path = format!("{}.{}.exif_value", resolved, i);
            let exif_value_bytes = &entries[i].entry;
            fs::write(exif_value_path, exif_value_bytes).expect("error writing exif value")
        }
    }

    let width = exif::get_tag_value(&exif, exif::ExifTagID::ImageWidth);

    println!("width: {:?}", width);

    let exif_path = format!("{}.exif", resolved);
    let exif_bytes = &exif.bytes;
    fs::write(exif_path, exif_bytes).expect("failed to write exif");

    let exif_path_no_header = format!("{}.exif_no_header", resolved);
    let exif_bytes_no_header = &exif.bytes[6..];
    fs::write(exif_path_no_header, exif_bytes_no_header).expect("failed to write exif w/o header");
}

fn main() {
    let resolved = parse_path(env::args().collect());
    let is_jpeg = jpeg_only(env::args().collect());

    let file = fs::read(&resolved).unwrap();

    if is_jpeg {
        process_jpeg(&file, resolved);
    } else {
        process_raw(file, resolved);
    }
}
