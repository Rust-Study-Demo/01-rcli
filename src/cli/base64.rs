use core::fmt;
use std::str::FromStr;

use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process_decode, process_encode, CmdExecutor};

use super::verify_file;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "decode a string to base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser=verify_file, default_value="-")]
    pub input: String,
    #[arg(long, value_parser=parse_base64_format, default_value="standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser=verify_file, default_value="-")]
    pub input: String,
    #[arg(long, value_parser=parse_base64_format, default_value="standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> anyhow::Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl CmdExecutor for Base64DecodeOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let decoded = process_decode(&self.input, self.format)?;
        //TODO: decoded data might not be string (but for this example, we assume it is)
        println!("{:?}", String::from_utf8(decoded));
        Ok(())
    }
}

impl CmdExecutor for Base64EncodeOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let encoded = process_encode(&self.input, self.format)?;
        println!("{}", encoded);
        Ok(())
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
