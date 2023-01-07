use crate::download::download_manager::download_tool;
use crate::download::download_manager::TOOLS_HOME;
use super::download_manager::ToolMetadata;

use simple_error::bail;
use std::error::Error;
use std::env;

#[derive(Debug)]
pub struct K8sMetadata {
    pub base_url: String,
    pub os: String,
    pub architecture: String,
    pub version: String,
    pub filename: String,
}

impl Default for K8sMetadata {
    fn default() -> K8sMetadata {
        K8sMetadata {
            base_url: String::from("https://dl.k8s.io/release"),
            os: String::from(env::consts::OS),
            architecture: String::from("amd64"),
            version: String::from("v1.26.0"),
            filename: String::from("kubectl"),
        }
    }
}

impl ToolMetadata for K8sMetadata {
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

    fn new_version(version: String) -> K8sMetadata {
        return K8sMetadata { version: version, ..Default::default() }
    }    
}

pub async fn do_k8s_download(metadata: &K8sMetadata) -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading with the following metadat: {:?}", metadata);
    let download_url = format!("{}/{}/bin/{}/{}/kubectl", metadata.base_url, metadata.version, metadata.os, metadata.architecture);
    return download_tool(download_url, metadata).await;
}
