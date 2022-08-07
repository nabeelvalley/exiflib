use rawlib::fuji;
use rawlib::jpeg;
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

fn main() {
    let resolved = parse_path(env::args().collect());
    let is_jpeg = jpeg_only(env::args().collect());

    let file = fs::read(&resolved).unwrap();

    if (is_jpeg) {
        println!("check some exif data");

        let width = jpeg::parse_tag_value(jpeg::ExifTagID::ImageWidth, &file);

        println!("width: {:?}", width);
    } else {
        let result = crate::fuji::parse(&file).expect("result does not exist");

        println!(
            "{} {} {} {}",
            result.format, result.identifier, result.model, result.version
        );

        let jpeg_path = format!("{}.jpeg", &resolved);

        fs::write(&jpeg_path, &result.jpeg.bytes).expect("failed to write jpeg");

        println!("jpeg written to {}", &jpeg_path);
    }
}
