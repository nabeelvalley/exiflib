use rawlib::fuji;
use std::env;
use std::fs;

fn parse_path(args: Vec<String>) -> String {
    let resolved = args.get(1);

    return resolved.expect("path was not provided").to_string();
}

fn main() {
    let resolved = parse_path(env::args().collect());

    let file = fs::read(&resolved).unwrap();

    let result = crate::fuji::parse(&file).expect("result does not exist");

    println!(
        "{} {} {} {}",
        result.format, result.identifier, result.model, result.version
    );

    let jpeg_path = format!("{}.jpeg", &resolved);

    fs::write(&jpeg_path, result.jpeg.bytes).expect("failed to write jpeg");

    println!("jpeg written to {}", &jpeg_path);
}
