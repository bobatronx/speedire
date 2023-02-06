use crate::metadata;
use crate::toolfs;

use std::error::Error;
use std::fs;
use std::process::{Command, Stdio, Output};
use simple_error::bail;

pub struct PoetryCommandBuilder {
    filename: String,
    version: String,
}

impl PoetryCommandBuilder {
    pub fn new() -> PoetryCommandBuilder {
        PoetryCommandBuilder { 
            filename: String::from("poetry"), 
            version: String::from("1.3.2"),
        }
    }

    pub fn compile(self) -> PoetryCommand {
        PoetryCommand {
            filename: self.filename,
            version: self.version,   
        }
    }
}

pub struct PoetryCommand {
    filename: String,
    version: String,
}

impl PoetryCommand {
    fn configure(&self) -> Result<(), Box<dyn Error>> {
        let tools_home = metadata::get_tools_home()?;
        let poetry_home = format!("{}/{}/{}", tools_home.tool_opt_dir, &self.filename, &self.version);

        setup_poetry_home(&poetry_home)
        .and(create_venv(&poetry_home))
        .and(install_poetry(&poetry_home, &self.version))?;
        
        Ok(())
    }
}

impl Default for PoetryCommand {
    fn default() -> PoetryCommand {
        let poetry = PoetryCommand {
            version: String::from("1.3.2"),
            filename: String::from("poetry"),
        };

        poetry.configure().expect("unable to configure poetry");

        poetry
    }
}

impl toolfs::BuilderTool for PoetryCommand {
    fn build(&self) -> Result<Output, Box<dyn Error>> {
        let tools_home = metadata::get_tools_home()?;
        let poetry_bin = format!("{}/{}/{}/bin/{}", tools_home.tool_opt_dir, self.filename, self.version, self.filename);
        
        Command::new(&poetry_bin)
        .arg("update")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .output()
        .expect("unable to execute poetry update command");

        Ok(Command::new(&poetry_bin)
        .arg("build")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .output()
        .expect("unable to execute poetry build command"))
    }
}


fn setup_poetry_home(poetry_home: &str) -> Result<(), Box<dyn Error>> {
    println!("creating poetry home directory: {}", poetry_home);
    fs::create_dir_all(poetry_home)?;

    Ok(())
}

fn create_venv(poetry_home: &str) -> Result<Output, Box<dyn Error>> {
    match Command::new("python3")
    .arg("-m")
    .arg("venv")
    .arg(poetry_home)
    .stdin(Stdio::null())
    .stdout(Stdio::inherit())
    .output() {
        Ok(o) => Ok(o),
        Err(e) => bail!("unable to build project {:?}", e) 
    }    
}

fn install_poetry(poetry_home: &str, poetry_version: &str) -> Result<Output, Box<dyn Error>> {
    let pip_location = format!("{}/bin/pip", poetry_home);
    match Command::new(&pip_location)
    .arg("install")
    .arg(format!("poetry=={}", poetry_version))
    .stdin(Stdio::null())
    .stdout(Stdio::inherit())
    .output() {
        Ok(o) => Ok(o),
        Err(e) => bail!("unable to run pip {:?}", e),
    }
}
