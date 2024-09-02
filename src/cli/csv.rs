use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use clap::Parser;

use crate::CmdExcetor;

use super::check_file_exist;

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

impl CmdExcetor for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output.clone()
        } else {
            format!("output.{}", self.format)
        };
        crate::process_csv(&self.input, &output, &self.format)
    }
}
