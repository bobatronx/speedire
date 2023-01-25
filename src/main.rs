// use carrier::tools::config::setup_tool_directory;
// use carrier::tools::config::setup_tool_permissions;
// use carrier::tools::config::ToolMetadata;
// use carrier::tools::kubectl_setup::do_kubectl_download;
// use carrier::tools::python_setup::do_python_download;
// use carrier::tools::kubectl_setup::KubectlMetadata;
// use carrier::tools::python_setup::PythonMetadata;
// use carrier::tools::python_setup::extract_python_tar;
// use carrier::tools::poetry_setup::PoetryMetadata;
// use carrier::tools::poetry_setup::do_poetry_download;
// use carrier::tools::poetry_setup::poetry_build;
use carrier::tools::{kubectl_setup::Kubectl, config::Tool};

fn main() {
    match carrier::tools::config::initialize() {
        Ok(_) => println!("carrier initialized successfully"),
        Err(e) => panic!("error initializing carrier {:?}", e),
    }
    
    let kubectl = Kubectl::default();
    match kubectl.configure() {
        Ok(_) => println!("kubectl configured successfully"),
        Err(e) => panic!("error downloading kubectl {:?}", e),
    }
    
    // match carrier::tools::config::cleanup() {
    //     Ok(_) => println!("spedire cleaned up successfully"),
    //     Err(e) => panic!("could not cleanup spedire tool directory {:?}", e),
    // }
    // let build_dir = "/home/bobatron/Development/projects/ella";
    // let output = poetry_build(build_dir, &["build"]);
    // match output {
    //     Ok(_) => println!("project built successfully"),
    //     Err(e) => panic!("error building project {:?}", e)
    // }

    // let poetry_metadata = PoetryMetadata::new_version("1.2.0");
    // match setup_tool_directory(&poetry_metadata) {
    //     Ok(_) => println!("poetry directory creatd successfully"),
    //     Err(e) => panic!("unable to setup tool directory {e:?}"),
    // }

    // match do_poetry_download(&poetry_metadata) {
    //     Ok(_) => println!("poetry download successfully"),
    //     Err(e) => panic!("error downloading poetry {e:?}"),
    // }
    
    // let k8s_metadata = KubectlMetadata::new_version("v1.26.0");
    
    // println!("setting up {} tool", k8s_metadata.filename);
    
    // println!("creating directory");
    // match setup_tool_directory(&k8s_metadata) {
    //     Ok(_) => println!("{} directory setup successfully", k8s_metadata.filename),
    //     Err(e) => panic!("unable to setup tool directory {e:?}"),
    // };

    // println!("downloading tool");    
    // match do_kubectl_download(&k8s_metadata) {
    //     Ok(_) => println!("{} downloaded successfully", k8s_metadata.filename),
    //     Err(e) => panic!("unable to download file! {e:?}"),
    // }

    // println!("setting permissions on tool");
    // match setup_tool_permissions(&k8s_metadata) {
    //     Ok(_) => println!("{} permissions set successfully", k8s_metadata.filename),
    //     Err(e) => panic!("unable to set permissions {e:?}"),
    // };

    // println!("{} is ready to use", k8s_metadata.filename);

    // let python_metadata = PythonMetadata::new_version("3.11.1");
    // match setup_tool_directory(&python_metadata) {
    //     Ok(_) => println!("{} directory setup successfully", python_metadata.filename),
    //     Err(e) => panic!("unable to create directory {e:?}"),
    // }

    // match do_python_download(&python_metadata) {
    //     Ok(_) => println!("{} downloaded successfully", python_metadata.filename),
    //     Err(e) => panic!("unable to download file {e:?}"),
    // }

    // match extract_python_tar(&python_metadata) {
    //     Ok(_) => println!("python extracted successfully"),
    //     Err(e) => println!("unable to extract python {e:?}"),
    // }
}
