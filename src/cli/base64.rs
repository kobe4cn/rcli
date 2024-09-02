use core::fmt;
use std::str::FromStr;

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::CmdExcetor;

use super::check_file_exist;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcetor)]
pub enum Base64SubCommand {
    /// Decode base64
    #[command(name = "decode", about = "Decode base64 string")]
    Decode(Base64DecodeOpts),
    /// Encode base64
    #[command(name = "encode", about = "Encode base64 string")]
    Encode(Base64EncodeOpts),
}
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    /// Input string
    #[arg(short, long, default_value = "-",value_parser=check_file_exist)]
    pub input: String,
    #[arg(long, default_value = "standard",value_parser=parse_base64_format)]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    /// Input string
    #[arg(short, long,default_value = "-",value_parser=check_file_exist)]
    pub input: String,
    #[arg(long, default_value = "standard",value_parser=parse_base64_format)]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(s: &str) -> Result<Base64Format, anyhow::Error> {
    s.parse::<Base64Format>()
}

impl From<Base64Format> for String {
    fn from(f: Base64Format) -> Self {
        match f {
            Base64Format::Standard => "standard".into(),
            Base64Format::UrlSafe => "url_safe".into(),
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "url_safe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::UrlSafe => write!(f, "url_safe"),
        }
    }
}

impl CmdExcetor for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encode = crate::process_encode(&self.input, self.format)?;
        println!("{}", encode);
        Ok(())
    }
}

impl CmdExcetor for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decode = crate::process_decode(&self.input, self.format)?;
        println!("{}", decode);
        Ok(())
    }
}
// impl CmdExcetor for Base64SubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             Base64SubCommand::Encode(opts) => opts.execute().await,
//             Base64SubCommand::Decode(opts) => opts.execute().await,
//         }
//     }
// }
