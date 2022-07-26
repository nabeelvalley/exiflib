use std::string::FromUtf8Error;

use crate::common::ImageFile;

pub fn parse(file: &Vec<u8>) -> Result<ImageFile, ()> {
    let format_identifier = parse_format_identifier(file);
    let camera_name = parse_camera_name(file);

    build_image_file(format_identifier, camera_name)
}

fn parse_format_identifier(file: &Vec<u8>) -> Result<String, FromUtf8Error> {
    let format_identifier = &file[0..16];
    String::from_utf8(format_identifier.to_vec())
}

fn parse_camera_name(file: &Vec<u8>) -> Result<String, FromUtf8Error> {
    let camera_name = &file[28..(28 + 32)];
    let parsed = String::from_utf8(camera_name.to_vec());

    match parsed {
        Ok(name) => {
            let sanitized = name.replace("\0", "");
            Ok(sanitized)
        }
        Err(e) => Err(e),
    }
}

fn build_image_file(
    format_identifier: Result<String, FromUtf8Error>,
    camera_name: Result<String, FromUtf8Error>,
) -> Result<ImageFile, ()> {
    match (format_identifier, camera_name) {
        (Ok(format), Ok(name)) => Ok(ImageFile {
            format,
            model: name.replace("\0", ""),
        }),
        _ => Err(()),
    }
}
