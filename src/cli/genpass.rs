use clap::Parser;

use crate::CmdExcetor;
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    /// Length of the password
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    /// Number of passwords to generate
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

impl CmdExcetor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = crate::process_genpass(self)?;
        println!("{}", password);
        let estimate = zxcvbn(&password, &[]);
        eprintln!("{:?}", estimate.score());
        Ok(())
    }
}
