use crate::tools::config;

use std::error::Error;

#[derive(Debug)]
pub struct Poetry {
    pub base_download_url: String,
    pub version: String,
    pub filename: String,
}

impl Default for Poetry {
    fn default() -> Poetry {
        Poetry {
            base_download_url: String::from("https://install.python-poetry.org"),
            version: String::from("1.2.0"),
            filename: String::from("poetry-install.sh"),
        }
    }
}

impl config::Tool for Poetry {

    fn configure(&self) -> Result<(), Box<dyn Error>> {
        todo!()
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