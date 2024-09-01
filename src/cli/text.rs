use core::fmt;
use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use super::{check_file_exist, verify_path};

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

    #[command(about = "Encrypt a message")]
    Encrypt(TextEncryptOpts),
    #[command(about = "Decrypt a message")]
    Decrypt(TextDecryptOpts),
    #[command(about = "Generate Cha Cha 20 key")]
    Chakey(ChaCha20KeyOpts),
}

#[derive(Debug, Parser)]
pub struct ChaCha20KeyOpts {
    #[arg(short,long, value_parser=verify_path)]
    pub output: PathBuf,
}
#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short,long, default_value = "-",value_parser=check_file_exist)]
    pub input: String,
    #[arg(short,long, value_parser=check_file_exist)]
    pub key: String,
    #[arg(short,long, value_parser=check_file_exist)]
    pub nonce: String,
}
#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short,long, default_value = "-",value_parser=check_file_exist)]
    pub input: String,
    #[arg(short,long, value_parser=check_file_exist)]
    pub key: String,
    #[arg(short,long, value_parser=check_file_exist)]
    pub nonce: String,
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
