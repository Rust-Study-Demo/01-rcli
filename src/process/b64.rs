use std::{fs::File, io::Read};

use crate::Base64Format;
use anyhow::{Ok, Result};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = read_data(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };

    println!("{}", encode);

    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = read_data(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // avoid accidental newlines
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    //TODO decoded data might not be string (but for this example, we assume it is)
    println!("{:?}", String::from_utf8(decoded));

    Ok(())
}

fn read_data(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
