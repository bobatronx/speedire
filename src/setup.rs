use std::io::Cursor;
use std::fs::File;
use std::error::Error;
use std::io::copy;
use std::process::Output;
use simple_error::bail;

pub fn download(url: &str, filepath: &str) -> Result<(), Box<dyn Error>> {
    let mut download = File::create(filepath)?;
    let resp = reqwest::blocking::get(url)?;
    let mut content = Cursor::new(resp.bytes()?);
    copy(&mut content, &mut download)?;

    Ok(())
}

const TOOLS_HOME: &str = ".local/spedire";
const TOOLS_BIN: &str = ".local/spedire/bin";
const TOOLS_TMP: &str = ".local/spedire/tmp";
const TOOLS_OPT: &str = ".local/spedire/opt";

pub struct ToolHome {
    pub tool_home: String,
    pub tool_bin_dir: String,
    pub tool_tmp_dir: String,
    pub tool_opt_dir: String,
}

pub trait Tool {
    fn configure(&self) -> Result<(), Box<dyn Error>>;
    fn execute_with_args(&self, args: &[&str]) -> Result<Output, Box<dyn Error>>;
    fn execute(&self, arg: &str) -> Result<Output, Box<dyn Error>>;
}

/// Get tool home information including the home directory,
/// the tmp directory, and the binary directory used with the
/// tool.
/// 
/// # Error
/// Errors if there are issues determining the home directory
pub fn get_tools_home() -> Result<ToolHome, Box<dyn Error>> {
    let home_dir = match home::home_dir() {
        Some(path) => path.display().to_string(),
        None => bail!("this tool requires a home directory setup at &HOME/.local/spedire, but home directory could not be detected based on os"),
    };
    
    return Ok(ToolHome {
        tool_home: format!("{}/{}", home_dir, TOOLS_HOME),
        tool_bin_dir: format!("{}/{}", home_dir, TOOLS_BIN),
        tool_tmp_dir: format!("{}/{}", home_dir, TOOLS_TMP),
        tool_opt_dir: format!("{}/{}", home_dir, TOOLS_OPT),
    })
}