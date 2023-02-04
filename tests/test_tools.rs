use speedire::kubectl_setup;
use speedire::toolfs;
use speedire::poetry_setup;
use std::{path::Path, fs};
use speedire::toolfs::Tool;

#[test]
fn test_initialize_cleanup() {
    let home_dir = home::home_dir().unwrap().display().to_string();
    let tool_dir = format!("{}/.local/speedire", home_dir);

    let tool_dir_path = Path::new(&tool_dir);

    if tool_dir_path.exists() {
        fs::remove_dir_all(&tool_dir).unwrap();
    }

    let initialize_result = toolfs::initialize();
    assert!(initialize_result.is_ok());
    assert!(Path::new(&tool_dir).exists());

    let cleanup_result = toolfs::cleanup();
    assert!(cleanup_result.is_ok());
    assert!(!Path::new(&tool_dir).exists());
}

#[test]
fn test_execute_poetry() {
    toolfs::initialize().unwrap();

    let poetry = poetry_setup::Poetry::default();
    poetry.configure().unwrap();
    let execute_result = poetry.execute("--version");
    assert!(execute_result.is_ok());

    toolfs::cleanup().unwrap();
}

#[test]
fn test_execute_kubectl() {
    toolfs::initialize().unwrap();

    let kubectl = kubectl_setup::Kubectl::default();
    kubectl.configure().unwrap();
    let set_namespace_result = kubectl.set_namespace("speedire");
    assert!(set_namespace_result.is_ok());
    let execute_result = kubectl.current_namespace();
    assert!(execute_result.is_ok());
    assert!(execute_result.unwrap() == "speedire");

    toolfs::cleanup().unwrap();
}

#[test]
fn test_pipeline_build() {

}