mod base64;
mod csv;
mod genpass;

use std::path::Path;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
};
use clap::Parser;
use csv::CsvOpts;
use genpass::GenPassOpts;

#[derive(Debug, Parser)]
#[command(name="cli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(file: &str) -> Result<String, String> {
    // if input is "-" or file exists
    if file == "-" || Path::new(file).exists() {
        Ok(file.into())
    } else {
        Err("File does not exist".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist".into()))
    }
}
