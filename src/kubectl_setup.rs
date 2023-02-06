use crate::metadata;
use crate::download_manager;
use crate::toolfs;

use std::error::Error;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio, Output};
use simple_error::bail;

pub struct KubectlCommand {
    base_download_url: String,
    os: String,
    architecture: String,
    version: String,
    filename: String,
    namespace: String,
    deploy_files: Vec<String>,
}

pub struct KubectlCommandBuilder {
    pub namespace: String,
    pub deploy_files: Vec<String>,
}

impl KubectlCommandBuilder {
    pub fn new() -> KubectlCommandBuilder {
        KubectlCommandBuilder { namespace: String::from("default"), deploy_files: Vec::new() }
    }

    pub fn namespace(mut self, namespace: &str) -> KubectlCommandBuilder {
        self.namespace = String::from(namespace);
        self
    }

    pub fn apply(mut self, filename: &str) -> KubectlCommandBuilder {
        self.deploy_files.push(String::from(filename));
        self
    }

    pub fn compile(self) -> KubectlCommand {
        KubectlCommand { 
            namespace: self.namespace, 
            deploy_files: self.deploy_files,
            ..Default::default()
        }
    }
}

impl KubectlCommand {

    fn configure(&self) -> Result<(), Box<dyn Error>> {
        let tools_home = metadata::get_tools_home()?;
        let download_url = format!("{}/{}/bin/{}/{}/kubectl", self.base_download_url, self.version, self.os, self.architecture);
        let download_location = tools_home.tool_tmp_dir + "/kubectl";
        let binary_location = format!("{}/{}/{}", &tools_home.tool_bin_dir, self.filename, self.version);
        let binary_file = format!("{}/{}", &binary_location, self.filename);

        println!("downloading kubectl from: {}", &download_url);
        println!("downloading kubectl to: {}", &download_location);

        download_manager::download(&download_url, &download_location)?;
        setup_kubectl_directories(&download_location, &binary_location, &binary_file)?;
        setup_kubectl_permissions(&binary_file)?;
    
        Ok(())
    }

    pub fn current_namespace(&self) -> Result<Output, Box<dyn Error>> {
        let tools_home = metadata::get_tools_home()?;
        let kubectl_bin = format!("{}/{}/{}/{}", tools_home.tool_bin_dir, &self.filename, &self.version, &self.filename);
        
        match Command::new(&kubectl_bin)
        .arg("config")
        .arg("view")
        .arg("--minify")
        .arg("-o")
        .arg("jsonpath={..namespace}")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output() {
            Ok(o) => Ok(o),
            Err(e) => bail!("could not get the current kubectl namespace {:?}", e),
        }
    }
}


impl Default for KubectlCommand {
    fn default() -> KubectlCommand {

        let kubectl = KubectlCommand {
            base_download_url: String::from("https://dl.k8s.io/release"),
            os: String::from(env::consts::OS),
            architecture: String::from("amd64"),
            version: String::from("v1.26.0"),
            filename: String::from("kubectl"),
            namespace: String::from("default"),
            deploy_files: Vec::new(),
        };

        kubectl.configure().expect("unable to configure kubectl");

        kubectl
    }
}

impl toolfs::DeployerTool for KubectlCommand {

    fn deploy(&self) -> Result<(), Box<dyn Error>> {
        let tools_home = metadata::get_tools_home()?;
        let kubectl_bin = format!("{}/{}/{}/{}", tools_home.tool_bin_dir, &self.filename, &self.version, &self.filename);
        
        match Command::new(&kubectl_bin)
        .arg("config")
        .arg("set-context")
        .arg("--current")
        .arg(format!("--namespace={}", self.namespace))
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .output() {
            Ok(_o) => (),
            Err(e) => bail!("could not change kubectl namespace {:?}", e),
        };

        for deploy_file in &self.deploy_files {
            match Command::new(&kubectl_bin)
            .arg("apply")
            .arg("-f")
            .arg(deploy_file)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .output() {
                Ok(_o) => (),
                Err(e) => bail!("could not kubectl apply files {:?}", e),
            };
        };

        Ok(())
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
