use carrier::tools::config;
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