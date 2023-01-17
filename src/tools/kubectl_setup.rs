use crate::download::download_manager::download;
use crate::tools::config::TOOLS_HOME;
use crate::tools::config::ToolMetadata;

use simple_error::bail;
use std::error::Error;
use std::env;

#[derive(Debug)]
pub struct KubectlMetadata {
    pub base_download_url: String,
    pub os: String,
    pub architecture: String,
    pub version: String,
    pub filename: String,
}

impl Default for KubectlMetadata {
    fn default() -> KubectlMetadata {
        KubectlMetadata {
            base_download_url: String::from("https://dl.k8s.io/release"),
            os: String::from(env::consts::OS),
            architecture: String::from("amd64"),
            version: String::from("v1.26.0"),
            filename: String::from("kubectl"),
        }
    }
}

impl ToolMetadata for KubectlMetadata {
    fn get_path_to_dir(&self) -> Result<String, Box<dyn Error>>  {
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

        return Ok(format!("{}/{}/{}/{}/{}", home_dir, TOOLS_HOME, self.filename, self.version, self.filename))
    }

    fn new_version(version: &str) -> KubectlMetadata {
        return KubectlMetadata { version: String::from(version), ..Default::default() }
    }    
}

pub fn do_kubectl_download(metadata: &KubectlMetadata) -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading with the following metadat: {:?}", metadata);
    let download_url = format!("{}/{}/bin/{}/{}/kubectl", metadata.base_download_url, metadata.version, metadata.os, metadata.architecture);
    return download(download_url, metadata.get_path_to_file()?);
}