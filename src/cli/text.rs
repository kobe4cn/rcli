use core::fmt;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::anyhow;
use clap::Parser;

use super::check_file_exist;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    /// Decode base64
    #[command(about = "sign a message with a private key")]
    Sign(TextSignOpts),
    /// Encode base64
    #[command(about = "verify a message with a public key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    Generate(TextKeyGenerateOps),
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOps {
    #[arg(long, default_value = "blake3",value_parser=parse_sign_format)]
    pub format: TextSignFormat,
    #[arg(short,long, value_parser=verify_path)]
    pub output: PathBuf,
}
#[derive(Debug, Parser)]
pub struct TextSignOpts {
    /// Input string
    #[arg(short, long, default_value = "-",value_parser=check_file_exist)]
    pub input: String,
    #[arg(long, value_parser=check_file_exist)]
    pub key: String,
    #[arg(long, default_value = "blake3",value_parser=parse_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    /// Input string
    #[arg(short, long,default_value = "-",value_parser=check_file_exist)]
    pub input: String,
    #[arg(long,value_parser=check_file_exist)]
    pub key: String,
    #[arg(long, short)]
    pub signature: String,
    #[arg(long, default_value = "blake3",value_parser=parse_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    ED25519,
}

fn parse_sign_format(s: &str) -> Result<TextSignFormat, anyhow::Error> {
    s.parse::<TextSignFormat>()
}

impl From<TextSignFormat> for String {
    fn from(f: TextSignFormat) -> Self {
        match f {
            TextSignFormat::Blake3 => "blake3".into(),
            TextSignFormat::ED25519 => "ed25519".into(),
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::ED25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "blake3"),
            TextSignFormat::ED25519 => write!(f, "ed25519"),
        }
    }
}

fn verify_path(s: &str) -> anyhow::Result<PathBuf> {
    let p = Path::new(s);
    if p.exists() && p.is_dir() {
        Ok(s.into())
    } else {
        Err(anyhow!("Path should not exist or is not a directory"))
    }
}
