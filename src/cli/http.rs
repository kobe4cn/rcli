use crate::CmdExcetor;

use super::verify_path;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcetor)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short,long, value_parser=verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExcetor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process_http_serve(self.dir.clone(), self.port).await?;
        Ok(())
    }
}
// impl CmdExcetor for HttpSubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             HttpSubCommand::Serve(opts) => opts.execute().await,
//         }
//     }
// }
