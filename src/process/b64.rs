use anyhow::Ok;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::cli::Base64Format;
use crate::reader_from_input;
pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let buffer = reader_from_input(input)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buffer),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buffer),
    };

    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let buffer = reader_from_input(input)?;
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buffer)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buffer)?,
    };

    Ok(String::from_utf8(decoded)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reader_from_input() {
        let input = "-";
        assert_eq!(reader_from_input(input).unwrap(), "".to_string());
    }
    #[test]
    fn test_process_encode() {
        let input = "-";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }
    #[test]
    fn test_process_decode() {
        let input = "fixture/tmp.b64";
        let format = Base64Format::Standard;
        assert!(process_decode(input, format).is_ok());
    }
}
