use std::io::Cursor;
use std::fs::{File, create_dir_all};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::error::Error;

pub const TOOLS_HOME: &str = ".local/bin";
const TOOL_DEFAULT_PERMS: u32 = 0o770;

pub trait ToolMetadata {
    fn get_path_to_dir(&self) -> Result<String, Box<dyn Error>>;
    fn get_path_to_file(&self) -> Result<String, Box<dyn Error>>;
    fn new_version(version: String) -> Self;
}

pub fn setup_tool_directory(tool_metadata: &impl ToolMetadata) -> Result<(), Box<dyn Error>> {
    let tool_dir = tool_metadata.get_path_to_dir()?;

    if !Path::new(&tool_dir).exists() {
        create_dir_all(&tool_dir)?;
    }

    return Ok(())
}

pub fn download_tool(url: String, tool_metadata: &impl ToolMetadata) -> Result<(), Box<dyn Error>> {
    let tool_file = tool_metadata.get_path_to_file()?;
    let mut download = File::create(tool_file)?;
    let resp = reqwest::blocking::get(url)?;
    let mut content = Cursor::new(resp.bytes()?);
    std::io::copy(&mut content, &mut download)?;
    
    Ok(())
}

pub fn setup_tool_permissions(tool_metadata: &impl ToolMetadata) -> Result<(), Box<dyn Error>> {
    let tool_file_path = tool_metadata.get_path_to_file()?;
    let tool_file = File::open(tool_file_path)?;

    let mut perms = tool_file.metadata()?.permissions();
    perms.set_mode(TOOL_DEFAULT_PERMS);
    tool_file.set_permissions(perms)?;
    
    Ok(())
}
