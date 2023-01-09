use crate::tools::config::ToolMetadata;
use crate::tools::config::TOOLS_HOME;
use crate::download::download_manager::download;

use tar::Archive;
use std::error::Error;
use std::fs::File;
use simple_error::bail;
use flate2::read::GzDecoder;

#[derive(Debug)]
pub struct PythonMetadata {
    pub base_download_url: String,
    pub version: String,
    pub filename: String,
}

impl Default for PythonMetadata {
    fn default() -> PythonMetadata {
        PythonMetadata {
            base_download_url: String::from("https://www.python.org/ftp/python"),
            version: String::from("3.11.1"),
            filename: String::from("python")
        }
    }
}

impl ToolMetadata for PythonMetadata {
    fn get_path_to_dir(&self) -> Result<String, Box<dyn Error>> {
        let home_dir = match home::home_dir() {
            Some(path) => path.display().to_string(),
            None => bail!("cannot find home directory"),
        };

        return Ok(format!("{}/{}/{}/{}", home_dir, TOOLS_HOME, self.filename, self.version))
    }

    fn get_path_to_file(&self) -> Result<String, Box<dyn Error>> {
        let home_dir = match home::home_dir() {
            Some(path) => path.display().to_string(),
            None => bail!("cannot find home directory"),
        };

        return Ok(format!("{}/{}/{}/{}/{}-{}.tgz", home_dir, TOOLS_HOME, self.filename, self.version, self.filename, self.version))
    }

    fn new_version(version: String) -> Self {
        return PythonMetadata{ version, ..Default::default() }
    }
}

pub fn do_python_download(metadata: &PythonMetadata) -> Result<(), Box<dyn Error>> {
    println!("Downloading with the following metadata: {:?}", metadata);
    let download_url = format!("{}/{}/Python-{}.tgz", metadata.base_download_url, metadata.version, metadata.version);
    return download(download_url, metadata.get_path_to_file()?);
}

pub fn extract_python_tar(metadata: &PythonMetadata) -> Result<(), Box<dyn Error>> {
    let python_tar_gz = File::open(metadata.get_path_to_file()?)?;
    let mut python_archive = Archive::new(GzDecoder::new(python_tar_gz));
    let maybe_unpack = python_archive.unpack(metadata.get_path_to_dir()?);

    match maybe_unpack {
        Ok(_) => Ok(()),
        Err(e) => bail!(e),
    }    
}