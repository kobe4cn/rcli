use core::fmt;
use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::CmdExcetor;

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

impl CmdExcetor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sig = crate::process_sign(&self.input, &self.key, self.format)?;
        println!("{}", sig);
        Ok(())
    }
}

impl CmdExcetor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::process_verify(&self.input, &self.key, &self.signature, self.format)?;
        println!("{}", ret);
        Ok(())
    }
}
impl CmdExcetor for TextKeyGenerateOps {
    async fn execute(self) -> anyhow::Result<()> {
        let key = crate::process_key_generate(self.format)?;
        match self.format {
            crate::TextSignFormat::Blake3 => {
                let name = &self.output.join("blake3.txt");
                std::fs::write(name, &key[0])?;
            }
            crate::TextSignFormat::ED25519 => {
                let name = &self.output;
                std::fs::write(name.join("ed25519.sk"), &key[0])?;
            }
        }
        Ok(())
    }
}

impl CmdExcetor for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::process_encrypt(&self.input, &self.key, &self.nonce)?;
        println!("{}", ret);
        Ok(())
    }
}

impl CmdExcetor for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::process_decrypt(&self.input, &self.key, &self.nonce)?;
        println!("{}", ret);
        Ok(())
    }
}

impl CmdExcetor for ChaCha20KeyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = crate::process_chacha_key_generate()?;
        println!("{:?}", key);
        Ok(())
    }
}

impl CmdExcetor for TextSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubCommand::Sign(opts) => opts.execute().await,
            TextSubCommand::Verify(opts) => opts.execute().await,
            TextSubCommand::Generate(opts) => opts.execute().await,
            TextSubCommand::Encrypt(opts) => opts.execute().await,
            TextSubCommand::Decrypt(opts) => opts.execute().await,
            TextSubCommand::Chakey(opts) => opts.execute().await,
        }
    }
}
