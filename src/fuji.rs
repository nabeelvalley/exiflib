use std::ops::Range;
use std::string::FromUtf8Error;

use crate::common::ImageFile;
use crate::helpers::bytes_to_string;

const FORMAT_RANGE: Range<usize> = 0..16;
const VERSION_RANGE: Range<usize> = 16..20;
const IDENTIFIER_RANGE: Range<usize> = 20..28;
const MODEL_RANGE: Range<usize> = 28..60;
const JPEG_OFFSET_RANGE: Range<usize> = 84..88;
const JPEG_LENGTH_RANGE: Range<usize> = 88..92;
const CFA_HEADER_OFFSET_RANGE: Range<usize> = 92..96;
const CFA_HEADER_LENGTH_RANGE: Range<usize> = 96..100;
const CFA_OFFSET_RANGE: Range<usize> = 100..104;
const CFA_LENGTH_RANGE: Range<usize> = 104..108;

pub fn parse(file: &Vec<u8>) -> Result<ImageFile, ()> {
    let model = parse_model(file);
    let format = bytes_to_string(file, FORMAT_RANGE);
    let identifier = bytes_to_string(file, IDENTIFIER_RANGE);
    let version = bytes_to_string(file, VERSION_RANGE);

    build_image_file(format, model, identifier, version)
}

fn parse_model(file: &Vec<u8>) -> Result<String, FromUtf8Error> {
    let parsed = bytes_to_string(file, MODEL_RANGE);

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
