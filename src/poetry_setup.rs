use crate::setup;

use std::error::Error;
use std::fs;
use std::process::{Command, Stdio, Output};
use simple_error::bail;

pub use setup::Tool;

#[derive(Debug)]
pub struct Poetry {
    pub version: String,
    pub filename: String,
}

impl Default for Poetry {
    fn default() -> Poetry {
        Poetry {
            version: String::from("1.3.2"),
            filename: String::from("poetry"),
        }
    }
}

impl Tool for Poetry {

    fn configure(&self) -> Result<(), Box<dyn Error>> {
        let tools_home = setup::get_tools_home()?;
        let poetry_home = format!("{}/{}/{}", tools_home.tool_opt_dir, &self.filename, &self.version);

        setup_poetry_home(&poetry_home)?;
        create_venv(&poetry_home)?;
        install_poetry(&poetry_home, &self.version)?;
    
        Ok(())
    }

    fn execute_with_args(&self, args: &[&str]) -> Result<Output, Box<dyn Error>> {
        let tools_home = setup::get_tools_home()?;
        let poetry_bin = format!("{}/{}/{}/bin/{}", tools_home.tool_opt_dir, &self.filename, &self.version, &self.filename);

        match Command::new(&poetry_bin)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .output() {
            Ok(o) => Ok(o),
            Err(e) => bail!("unable to execute poetry command {:?}", e),
        }
    }

    fn execute(&self, arg: &str) -> Result<Output, Box<dyn Error>> {
        match self.execute_with_args(&[arg]) {
            Ok(o) => Ok(o),
            Err(e) => bail!("unable to execute poetry command {:?}", e),
        }  
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
// impl ToolMetadata for PoetryMetadata {
//     fn get_path_to_dir(&self) -> Result<String, Box<dyn Error>> {
//         let home_dir = match home::home_dir() {
//             Some(path) => path.display().to_string(),
//             None => bail!("cannot find home directory"),
//         };

//         return Ok(format!("{}/{}/poetry/{}", home_dir, TOOLS_HOME, self.version))
//     }

//     fn get_path_to_file(&self) -> Result<String, Box<dyn Error>> {
//         let home_dir = match home::home_dir() {
//             Some(path) => path.display().to_string(),
//             None => bail!("cannot find home directory"),
//         };

//         return Ok(format!("{}/{}/poetry/{}/{}", home_dir, TOOLS_HOME, self.version, self.filename))
//     }

//     fn new_version(version: &str) -> Self {
//         return PoetryMetadata { version: String::from(version), ..Default::default() }
//     }
// }

// pub fn do_poetry_download(metadata: &PoetryMetadata) -> Result<(), Box<dyn Error>> {
//     let download_url = format!("{}", metadata.base_download_url);
//     return download(download_url.as_str(), metadata.get_path_to_file()?);
// }

// pub fn poetry_build(project_dir: &str, args: &[&str]) -> Result<Output, Box<dyn Error>> {
//     match Command::new("poetry")
//     .args(args)
//     .stdin(Stdio::null())
//     .stdout(Stdio::inherit())
//     .current_dir(project_dir)
//     .output() {
//         Ok(o) => Ok(o),
//         Err(e) => bail!("unable to build project {:?}", e) 
//     }
// }