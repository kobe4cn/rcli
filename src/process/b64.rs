use std::io::Read;

use anyhow::Ok;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::cli::Base64Format;
fn reader_from_input(input: &str) -> anyhow::Result<String> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    //avoid trailing newline
    buffer = buffer.trim().to_string();
    Ok(buffer)
}
pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let buffer = reader_from_input(input)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buffer),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buffer),
    };

    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let buffer = reader_from_input(input)?;
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buffer)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buffer)?,
    };
    println!("{}", String::from_utf8(decoded)?);
    Ok(())
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
