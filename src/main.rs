use crate::download::download_manager::ToolMetadata;
use crate::download::k8s_downloader::do_k8s_download;
use crate::download::k8s_downloader::K8sMetadata;
use crate::download::download_manager::setup_tool_directory;
use crate::download::download_manager::setup_tool_permissions;

mod download;

#[tokio::main]
async fn main() {
    
    let k8s_metadata = K8sMetadata::new_version(String::from("v1.25.0"));
    
    println!("setting up {} tool", k8s_metadata.filename);
    
    println!("creating directory");
    match setup_tool_directory(&k8s_metadata) {
        Ok(_) => println!("{} directory setup successfully", k8s_metadata.filename),
        Err(e) => panic!("unable to setup tool directory {e:?}"),
    };


    println!("downloading tool");    
    match do_k8s_download(&k8s_metadata).await {
        Ok(_) => println!("{} downloaded successfully", k8s_metadata.filename),
        Err(e) => panic!("unable to download file! {e:?}"),
    }

    println!("setting permissions on tool");
    match setup_tool_permissions(&k8s_metadata) {
        Ok(_) => println!("{} permissions set successfully", k8s_metadata.filename),
        Err(e) => panic!("unable to set permissions {e:?}"),
    };

    println!("{} is ready to use", k8s_metadata.filename);
}
