use crate::tools::config::setup_tool_directory;
use crate::tools::config::setup_tool_permissions;
use crate::tools::config::ToolMetadata;
use crate::tools::kubectl_setup::do_kubectl_download;
use crate::tools::python_setup::do_python_download;
use crate::tools::kubectl_setup::KubectlMetadata;
use crate::tools::python_setup::PythonMetadata;
use crate::tools::python_setup::extract_python_tar;

mod download;
mod tools;

fn main() {
    
    let k8s_metadata = KubectlMetadata::new_version(String::from("v1.26.0"));
    
    println!("setting up {} tool", k8s_metadata.filename);
    
    println!("creating directory");
    match setup_tool_directory(&k8s_metadata) {
        Ok(_) => println!("{} directory setup successfully", k8s_metadata.filename),
        Err(e) => panic!("unable to setup tool directory {e:?}"),
    };

    println!("downloading tool");    
    match do_kubectl_download(&k8s_metadata) {
        Ok(_) => println!("{} downloaded successfully", k8s_metadata.filename),
        Err(e) => panic!("unable to download file! {e:?}"),
    }

    println!("setting permissions on tool");
    match setup_tool_permissions(&k8s_metadata) {
        Ok(_) => println!("{} permissions set successfully", k8s_metadata.filename),
        Err(e) => panic!("unable to set permissions {e:?}"),
    };

    println!("{} is ready to use", k8s_metadata.filename);

    let python_metadata = PythonMetadata::new_version(String::from("3.11.1"));
    match setup_tool_directory(&python_metadata) {
        Ok(_) => println!("{} directory setup successfully", python_metadata.filename),
        Err(e) => panic!("unable to create directory {e:?}"),
    }

    match do_python_download(&python_metadata) {
        Ok(_) => println!("{} downloaded successfully", python_metadata.filename),
        Err(e) => panic!("unable to download file {e:?}"),
    }

    match extract_python_tar(&python_metadata) {
        Ok(_) => println!("python extracted successfully"),
        Err(e) => println!("unable to extract python {e:?}"),
    }
}
