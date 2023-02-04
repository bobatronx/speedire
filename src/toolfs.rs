use crate::metadata;

use std::path::Path;
use std::fs;
use std::error::Error;
use std::process::Output;

pub trait Tool {
    fn configure(&self) -> Result<(), Box<dyn Error>>;
}

pub trait BuilderTool {
    // fn execute_with_args(&self, args: &[&str]) -> Box<dyn Tool>;
    // fn execute(&self, arg: &str) -> Box<dyn Tool>;
    fn build(&self) -> Result<Output, Box<dyn Error>>;
}

pub trait DeployerTool {
    fn deploy(&self, args: &[&str]) -> Result<Output, Box<dyn Error>>;
}

/// Initialize the Spedire tool system by creating the temporary
/// download diretory, the bin directory and the opt directory 
/// for tool executables and versions
/// 
/// # Errors
/// Errors due to any issue working with the file system
pub fn initialize() -> Result<(), Box<dyn Error>> {
    let tools_home = metadata::get_tools_home()?;

    if !Path::new(&tools_home.tool_tmp_dir).exists() {
        println!("creating spedire tmp dir: {}", &tools_home.tool_tmp_dir);
        fs::create_dir_all(&tools_home.tool_tmp_dir)?;
    }

    if !Path::new(&tools_home.tool_bin_dir).exists() {
        println!("creating spedire bin dir: {}", &tools_home.tool_tmp_dir);
        fs::create_dir_all(&tools_home.tool_bin_dir)?;
    }

    if !Path::new(&tools_home.tool_opt_dir).exists() {
        println!("creating spedire opt dir: {}", &tools_home.tool_opt_dir);
        fs::create_dir_all(&tools_home.tool_opt_dir)?;
    }

    return Ok(())
}

/// Cleanup the temporary download directory and the bin directory
/// that are created as part of initializing the Spedire tool.
/// 
/// # Errors
/// Errors due to any issues with the filesystem
pub fn cleanup() -> Result<(), Box<dyn Error>> {
    let tools_home = metadata::get_tools_home()?;

    if Path::new(&tools_home.tool_home).exists() {
        println!("removing speedire working directory: {}", &tools_home.tool_home);
        fs::remove_dir_all(&tools_home.tool_home)?;
    }

    return Ok(())
}