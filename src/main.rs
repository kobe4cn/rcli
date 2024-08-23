// rcli csv -i input.csv -o output.json --header -d ','

use std::{fs, path::Path};

use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about,long_about=None)]
struct Opts {
    #[command(subcommand)]
    cmd: Subcommand,
}

#[derive(Debug, Parser)]
enum Subcommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    /// Input file path
    #[arg(short, long, value_parser=check_file_exist)]
    input: String,

    /// Output file path
    #[arg(short, long, default_value = "output.json")]
    output: String,

    /// CSV delimiter
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    /// CSV has header or not
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn check_file_exist(s: &str) -> Result<String, String> {
    if Path::new(s).exists() {
        Ok(s.into())
    } else {
        Err("File does not exist".into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,

    position: String,
    #[serde(rename = "DOB")]
    dob: String,

    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            let mut ret = Vec::with_capacity(128);
            // let records= reader.deserialize().map(|record| record.unwrap()).collect::<Vec<Player>>();
            for result in reader.deserialize() {
                let record: Player = result?;
                ret.push(record);
            }
            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(opts.output, json)?;
        }
    }

    // print!("{:?}", opts);
    Ok(())
}
