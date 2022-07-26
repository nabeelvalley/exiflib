use rawlib::fuji;
use std::env;
use std::fs;

fn parse_path(args: Vec<String>) -> String {
    let resolved = args.get(1);

    return resolved.expect("path was not provided").to_string();
}

fn main() {
    let resolved = parse_path(env::args().collect());

    let file = fs::read(resolved).unwrap();

    let result = crate::fuji::parse(&file);

    let result = result.unwrap();

    println!("{} {}", result.format, result.model);
}
