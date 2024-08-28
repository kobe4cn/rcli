mod csv;
mod genpass;

pub use self::{csv::CsvOpts, csv::OutputFormat, genpass::GenPassOpts};

use clap::Parser;

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
    // #[command(name="base64",about="decode and encode base64")]
    // Base64(Base64Opts),
}
