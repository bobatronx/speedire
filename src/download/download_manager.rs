use std::io::Cursor;
use std::fs::File;
use std::error::Error;
use std::io::copy;

pub fn download(url: String, filepath: String) -> Result<(), Box<dyn Error>> {
    let mut download = File::create(filepath)?;
    let resp = reqwest::blocking::get(url)?;
    let mut content = Cursor::new(resp.bytes()?);
    copy(&mut content, &mut download)?;

    Ok(())
}
