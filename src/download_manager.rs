use std::error::Error;
use std::fs::File;
use std::io::Cursor;
use std::io;

pub fn download(url: &str, filepath: &str) -> Result<(), Box<dyn Error>> {
    let mut download = File::create(filepath)?;
    let resp = reqwest::blocking::get(url)?;
    let mut content = Cursor::new(resp.bytes()?);
    io::copy(&mut content, &mut download)?;

    Ok(())
}