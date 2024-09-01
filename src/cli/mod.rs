mod base64;
mod csv;
mod genpass;
mod jwt;
mod text;

use std::path::{Path, PathBuf};

pub use self::base64::{Base64Format, Base64SubCommand};
use self::csv::CsvOpts;
pub use self::csv::OutputFormat;
pub use self::genpass::GenPassOpts;
pub use self::text::{TextSignFormat, TextSubCommand};
use anyhow::anyhow;
use clap::Parser;
pub use jwt::JwtSubCommand;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Jwt(JwtSubCommand),
}

fn check_file_exist(s: &str) -> Result<String, String> {
    if s == "-" || Path::new(s).exists() {
        Ok(s.into())
    } else {
        Err("File does not exist".into())
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_file_exist() {
        assert_eq!(check_file_exist("-"), Ok("-".into()));
        assert_eq!(check_file_exist("*"), Err("File does not exist".into()));
        assert_eq!(check_file_exist("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            check_file_exist("not-exist"),
            Err("File does not exist".into())
        );
    }
}
