use std::ops::Range;
use std::string::FromUtf8Error;

use crate::common::ImageFile;
use crate::helpers::{bytes_to_integer_be, bytes_to_string};

const FORMAT_RANGE: Range<usize> = 0..16;
const VERSION_RANGE: Range<usize> = 16..20;
const IDENTIFIER_RANGE: Range<usize> = 20..28;
const MODEL_RANGE: Range<usize> = 28..60;
const OFFSET_DIRECTORY_VERSION: Range<usize> = 60..64;
const JPEG_OFFSET_RANGE: Range<usize> = 84..88;
const JPEG_LENGTH_RANGE: Range<usize> = 88..92;
const CFA_HEADER_OFFSET_RANGE: Range<usize> = 92..96;
const CFA_HEADER_LENGTH_RANGE: Range<usize> = 96..100;
const CFA_OFFSET_RANGE: Range<usize> = 100..104;
const CFA_LENGTH_RANGE: Range<usize> = 104..108;

pub fn parse(bytes: &Vec<u8>) -> Result<ImageFile, ()> {
    let model = parse_model(bytes);
    let format = parse_format(bytes);
    let identifier = parse_identifier(bytes);
    let version = parse_version(bytes);

    debug_info(&bytes);

    build_image_file(format, model, identifier, version)
}

// TODO: remove this obviously
fn debug_info(bytes: &Vec<u8>) {
    println!(
        "OFFSET_DIRECTORY_VERSION {:?}",
        bytes_to_string(bytes, OFFSET_DIRECTORY_VERSION)
    );
    println!(
        "JPEG_OFFSET_RANGE {:?}",
        bytes_to_integer_be(bytes, JPEG_OFFSET_RANGE)
    );
    println!(
        "JPEG_LENGTH_RANGE {:?}",
        bytes_to_integer_be(bytes, JPEG_LENGTH_RANGE)
    );
    println!(
        "CFA_HEADER_OFFSET_RANGE {:?}",
        bytes_to_integer_be(bytes, CFA_HEADER_OFFSET_RANGE)
    );
    println!(
        "CFA_HEADER_LENGTH_RANGE {:?}",
        bytes_to_integer_be(bytes, CFA_HEADER_LENGTH_RANGE)
    );
    println!(
        "CFA_OFFSET_RANGE {:?}",
        bytes_to_integer_be(bytes, CFA_OFFSET_RANGE)
    );
    println!(
        "CFA_LENGTH_RANGE {:?}",
        bytes_to_integer_be(bytes, CFA_LENGTH_RANGE)
    );
}

fn parse_model(bytes: &Vec<u8>) -> Result<String, FromUtf8Error> {
    let parsed = bytes_to_string(bytes, MODEL_RANGE);

    match parsed {
        Ok(name) => {
            let sanitized = name.replace("\0", "");
            Ok(sanitized)
        }
        Err(e) => Err(e),
    }
}

fn build_image_file(
    format_result: Result<String, FromUtf8Error>,
    model_result: Result<String, FromUtf8Error>,
    identifier_result: Result<String, FromUtf8Error>,
    version_result: Result<String, FromUtf8Error>,
) -> Result<ImageFile, ()> {
    match (
        format_result,
        model_result,
        identifier_result,
        version_result,
    ) {
        // TODO: there's probably a better way to do this
        (Ok(format), Ok(model), Ok(identifier), Ok(version)) => Ok(ImageFile {
            format,
            model,
            identifier,
            version,
        }),
        _ => Err(()),
    }
}

fn parse_version(bytes: &Vec<u8>) -> Result<String, FromUtf8Error> {
    bytes_to_string(bytes, VERSION_RANGE)
}

fn parse_identifier(bytes: &Vec<u8>) -> Result<String, FromUtf8Error> {
    bytes_to_string(bytes, IDENTIFIER_RANGE)
}

fn parse_format(bytes: &Vec<u8>) -> Result<String, FromUtf8Error> {
    bytes_to_string(bytes, FORMAT_RANGE)
}
