mod cli;
mod process;

pub use cli::{Opts, Subcommand};

pub use process::process_csv;
pub use process::process_genpass;
