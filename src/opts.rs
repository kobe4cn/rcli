use clap::Parser;
use core::fmt;
use std::{
    fmt::{Display, Formatter},
    path::Path,
    str::FromStr,
};

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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// Input file path
    #[arg(short, long, value_parser=check_file_exist)]
    pub input: String,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<String>,

    /// CSV delimiter
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    /// CSV has header or not
    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(long,value_parser=parse_format, default_value = "json")]
    pub format: OutputFormat,
}

fn check_file_exist(s: &str) -> Result<String, String> {
    if Path::new(s).exists() {
        Ok(s.into())
    } else {
        Err("File does not exist".into())
    }
}

fn parse_format(s: &str) -> Result<OutputFormat, anyhow::Error> {
    s.parse::<OutputFormat>()
}

impl From<OutputFormat> for String {
    fn from(f: OutputFormat) -> Self {
        match f {
            OutputFormat::Json => "json".into(),
            OutputFormat::Yaml => "yaml".into(),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}
