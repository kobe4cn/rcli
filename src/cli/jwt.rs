use std::path::PathBuf;

use clap::Parser;

use crate::CmdExcetor;

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

impl CmdExcetor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = crate::process_jwt_sign(&self.key, &self.sub, &self.aud, &self.exp)?;
        println!("{}", token);
        Ok(())
    }
}

impl CmdExcetor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::process_jwt_verify(&self.key, &self.token)?;
        println!("{:?}", ret);
        Ok(())
    }
}

impl CmdExcetor for JwtSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            JwtSubCommand::Sign(opts) => opts.execute().await,
            JwtSubCommand::Verify(opts) => opts.execute().await,
        }
    }
}
