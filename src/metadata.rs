use std::error::Error;
use simple_error::bail;

const TOOLS_HOME: &str = ".local/speedire";
const TOOLS_BIN: &str = ".local/speedire/bin";
const TOOLS_TMP: &str = ".local/speedire/tmp";
const TOOLS_OPT: &str = ".local/speedire/opt";

pub struct ToolHome {
    pub tool_home: String,
    pub tool_bin_dir: String,
    pub tool_tmp_dir: String,
    pub tool_opt_dir: String,
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
        None => bail!("this tool requires a home directory setup at &HOME/.local/speedire, but home directory could not be detected based on os"),
    };
    
    return Ok(ToolHome {
        tool_home: format!("{}/{}", home_dir, TOOLS_HOME),
        tool_bin_dir: format!("{}/{}", home_dir, TOOLS_BIN),
        tool_tmp_dir: format!("{}/{}", home_dir, TOOLS_TMP),
        tool_opt_dir: format!("{}/{}", home_dir, TOOLS_OPT),
    })
}