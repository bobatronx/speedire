use crate::metadata;
use crate::download_manager;
use crate::toolfs;

use std::error::Error;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Output;

#[derive(Debug)]
pub struct Kubectl {
    pub base_download_url: String,
    pub os: String,
    pub architecture: String,
    pub version: String,
    pub filename: String,
}

impl Default for Kubectl {
    fn default() -> Kubectl {
        Kubectl {
            base_download_url: String::from("https://dl.k8s.io/release"),
            os: String::from(env::consts::OS),
            architecture: String::from("amd64"),
            version: String::from("v1.26.0"),
            filename: String::from("kubectl"),
        }
    }
}

impl toolfs::Tool for Kubectl {

    fn configure(&self) -> Result<(), Box<dyn Error>> {
        let tools_home = metadata::get_tools_home()?;
        let download_url = format!("{}/{}/bin/{}/{}/kubectl", &self.base_download_url, &self.version, &self.os, &self.architecture);
        let download_location = tools_home.tool_tmp_dir + "/kubectl";
        let binary_location = format!("{}/{}/{}", &tools_home.tool_bin_dir, &self.filename, &self.version);
        let binary_file = format!("{}/{}", &binary_location, &self.filename);

        println!("downloading kubectl from: {}", &download_url);
        println!("downloading kubectl to: {}", &download_location);

        download_manager::download(&download_url, &download_location)?;
        setup_kubectl_directories(&download_location, &binary_location, &binary_file)?;
        setup_kubectl_permissions(&binary_file)?;
    
        Ok(())
    }

    fn execute(&self, _arg: &str) -> Result<Output, Box<dyn Error>> {
        todo!()
    }

    fn execute_with_args(&self, _args: &[&str]) -> Result<Output, Box<dyn Error>> {
        todo!()
    }
}

fn setup_kubectl_directories(download_location: &str, binary_location: &str, binary_file: &str) -> Result<(), Box<dyn Error>> {
    println!("creating directory: {}", binary_location);
    fs::create_dir_all(binary_location)?;
    println!("creating file: {}", binary_file);
    fs::File::create(binary_file)?;
    println!("copying kubectl to: {}", binary_file);
    fs::copy(download_location, binary_file)?;

    Ok(())
}

fn setup_kubectl_permissions(kubectl_binary_location: &str) -> Result<(), Box<dyn Error>> {
    println!("setting execute permissions on kubectl");
    let kubectl = fs::File::open(kubectl_binary_location)?;
    let mut perms = kubectl.metadata()?.permissions();
    perms.set_mode(0o770);
    kubectl.set_permissions(perms)?;

    Ok(())
}
