use carrier::tools::config;
use carrier::tools::config::Tool;
use carrier::tools::poetry_setup;
use std::{path::Path, fs};

#[test]
fn test_initialize_cleanup() {
    let home_dir = home::home_dir().unwrap().display().to_string();
    let tool_dir = format!("{}/.local/spedire", home_dir);

    let tool_dir_path = Path::new(&tool_dir);

    if tool_dir_path.exists() {
        fs::remove_dir_all(&tool_dir).unwrap();
    }

    let initialize_result = config::initialize();
    assert!(initialize_result.is_ok());
    assert!(Path::new(&tool_dir).exists());

    let cleanup_result = config::cleanup();
    assert!(cleanup_result.is_ok());
    assert!(!Path::new(&tool_dir).exists());
}

#[test]
fn test_execute_poetry() {
    let home_dir = home::home_dir().unwrap().display().to_string();
    let tool_dir = format!("{}/.local/spedire/opt/poetry", home_dir);

    let poetry = poetry_setup::Poetry::default();
    poetry.configure().unwrap();
    let execute_result = poetry.execute("--version");
    assert!(execute_result.is_ok());

    fs::remove_dir_all(&tool_dir).unwrap();   
}
