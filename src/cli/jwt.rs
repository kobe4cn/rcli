use std::path::PathBuf;

use clap::Parser;

use super::{check_file_exist, verify_path};

#[derive(Debug, Parser)]
pub enum JwtSubCommand {
    #[command(about = "sign the jwt token")]
    Sign(JwtSignOpts),
    #[command(about = "verify JWT token")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtKeyGenerateOps {
    #[arg(short,long, value_parser=verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long, default_value = "-",value_parser=check_file_exist)]
    pub key: String,
    #[arg(long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long)]
    pub exp: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(long, default_value = "-",value_parser=check_file_exist)]
    pub key: String,
    #[arg(long, short)]
    pub token: String,
}
