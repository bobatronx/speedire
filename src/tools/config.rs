use std::fs;
use std::path::Path;
use std::error::Error;
use simple_error::bail;

const TOOLS_HOME: &str = ".local/spedire";
const TOOLS_BIN: &str = ".local/spedire/bin";
const TOOLS_TMP: &str = ".local/spedire/tmp";

pub struct ToolHome {
    pub tool_home: String,
    pub tool_bin_dir: String,
    pub tool_tmp_dir: String,
}

pub trait Tool {
    fn configure(&self) -> Result<(), Box<dyn Error>>;
    // fn execute(&self) -> Result<(), Box<dyn Error>>;
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
    })
}

/// Initialize the Spedire tool system by creating the temporary
/// download diretory and the bin directory for tool executables
/// and versions
/// 
/// # Errors
/// Errors due to any issue working with the file system
pub fn initialize() -> Result<(), Box<dyn Error>> {
    let tools_home = get_tools_home()?;

    if !Path::new(&tools_home.tool_tmp_dir).exists() {
        println!("creating spedire tmp dir: {}", &tools_home.tool_tmp_dir);
        fs::create_dir_all(&tools_home.tool_tmp_dir)?;
    }

    if !Path::new(&tools_home.tool_bin_dir).exists() {
        println!("creating spedire bin dir: {}", &tools_home.tool_tmp_dir);
        fs::create_dir_all(&tools_home.tool_bin_dir)?;
    }

    return Ok(())
}

/// Cleanup the temporary download directory and the bin directory
/// that are created as part of initializing the Spedire tool.
/// 
/// # Errors
/// Errors due to any issues with the filesystem
pub fn cleanup() -> Result<(), Box<dyn Error>> {
    let tools_home = get_tools_home()?;

    if Path::new(&tools_home.tool_home).exists() {
        println!("removing spedire working directory: {}", &tools_home.tool_home);
        fs::remove_dir_all(&tools_home.tool_home)?;
    }

    return Ok(())
}
