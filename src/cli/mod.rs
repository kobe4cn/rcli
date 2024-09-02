mod base64;
mod csv;
mod genpass;
mod http;
mod jwt;
mod text;

use std::path::{Path, PathBuf};

pub use self::base64::*;
pub use self::csv::*;
pub use self::genpass::*;
pub use self::http::*;
pub use self::jwt::*;
pub use self::text::*;

use anyhow::{anyhow, Ok};
use clap::Parser;
use enum_dispatch::enum_dispatch;
pub use jwt::JwtSubCommand;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcetor)]
pub enum Subcommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "JWT sign/verify")]
    Jwt(JwtSubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}
// impl CmdExcetor for Subcommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             Subcommand::Csv(opts) => opts.execute().await,

//             Subcommand::GenPass(opts) => opts.execute().await,
//             Subcommand::Base64(subcmd) => subcmd.execute().await,
//             Subcommand::Text(subcmd) => subcmd.execute().await,
//             Subcommand::Jwt(subcmd) => subcmd.execute().await,
//             Subcommand::Http(subcmd) => subcmd.execute().await,
//         }
//     }
// }

fn check_file_exist(s: &str) -> anyhow::Result<String> {
    if s == "-" || Path::new(s).exists() {
        Ok(s.into())
    } else {
        Err(anyhow!("File does not exist"))
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_check_file_exist() {
//         assert_eq!(check_file_exist("-"), Ok("-".into()));
//         assert_eq!(check_file_exist("*"), Err("File does not exist".into()));
//         assert_eq!(check_file_exist("Cargo.toml"), Ok("Cargo.toml".into()));
//         assert_eq!(
//             check_file_exist("not-exist"),
//             Err("File does not exist".into())
//         );
//     }
// }
