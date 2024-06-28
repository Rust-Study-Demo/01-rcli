mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::Path;

pub use self::{base64::*, csv::*, genpass::*, http::*, text::*};

#[derive(Debug, Parser)]
#[command(name="cli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

fn verify_file(file: &str) -> Result<String, String> {
    // if input is "-" or file exists
    if file == "-" || Path::new(file).exists() {
        Ok(file.into())
    } else {
        Err("File does not exist".into())
    }
}

fn verify_path(path: &str) -> Result<String, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("path dose not exists or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist".into()))
    }
}
