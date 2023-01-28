use crate::setup;

use std::error::Error;

pub use setup::Tool;

#[derive(Debug)]
pub struct Python {
    pub base_download_url: String,
    pub version: String,
    pub filename: String,
}

impl Default for Python {
    fn default() -> Python {
        Python {
            base_download_url: String::from("https://www.python.org/ftp/python"),
            version: String::from("3.11.1"),
            filename: String::from("python"),
        }
    }
}

impl setup::Tool for Python {

    fn configure(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn execute(&self, _arg: &str) -> Result<std::process::Output, Box<dyn Error>> {
        todo!()
    }

    fn execute_with_args(&self, _args: &[&str]) -> Result<std::process::Output, Box<dyn Error>> {
        todo!()
    }
}

// impl ToolMetadata for PythonMetadata {
//     fn get_path_to_dir(&self) -> Result<String, Box<dyn Error>> {
//         let home_dir = match home::home_dir() {
//             Some(path) => path.display().to_string(),
//             None => bail!("cannot find home directory"),
//         };

//         return Ok(format!("{}/{}/{}/{}", home_dir, TOOLS_HOME, self.filename, self.version))
//     }

//     fn get_path_to_file(&self) -> Result<String, Box<dyn Error>> {
//         let home_dir = match home::home_dir() {
//             Some(path) => path.display().to_string(),
//             None => bail!("cannot find home directory"),
//         };

//         return Ok(format!("{}/{}/{}/{}/{}-{}.tgz", home_dir, TOOLS_HOME, self.filename, self.version, self.filename, self.version))
//     }

//     fn new_version(version: &str) -> Self {
//         return PythonMetadata { version: String::from(version), ..Default::default() }
//     }
// }

// pub fn do_python_download(metadata: &PythonMetadata) -> Result<(), Box<dyn Error>> {
//     let download_url = format!("{}/{}/Python-{}.tgz", metadata.base_download_url, metadata.version, metadata.version);
//     return download(download_url.as_str(), metadata.get_path_to_file()?);
// }

// pub fn extract_python_tar(metadata: &PythonMetadata) -> Result<(), Box<dyn Error>> {
//     let python_tar_gz = File::open(metadata.get_path_to_file()?)?;
//     let mut python_archive = Archive::new(GzDecoder::new(python_tar_gz));
//     let maybe_unpack = python_archive.unpack(metadata.get_path_to_dir()?);

//     match maybe_unpack {
//         Ok(_) => {
//             match remove_file(metadata.get_path_to_file()?) {
//                 Ok(_) => Ok(()),
//                 Err(e) => {
//                     println!("unable to remove python tar file after extracting it {:#?}", e);
//                     Ok(())
//                 }
//             }
//         },
//         Err(e) => bail!(e),
//     }    
// }
