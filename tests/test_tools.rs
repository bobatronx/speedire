use speedire::commands::kubectl::{KubectlCommandBuilder};
use speedire::commands::poetry::{PoetryCommandBuilder};
use std::{path::Path, fs};
use speedire::pipelines::*;

#[test]
fn test_initialize_cleanup() {
    let home_dir = home::home_dir().unwrap().display().to_string();
    let tool_dir = format!("{}/.local/speedire", home_dir);

    let tool_dir_path = Path::new(&tool_dir);

    if tool_dir_path.exists() {
        fs::remove_dir_all(&tool_dir).unwrap();
    }

    Speedire::new();
    assert!(Path::new(&tool_dir).exists());
    Speedire::destroy();
    assert!(!Path::new(&tool_dir).exists());
}

#[test]
fn test_execute_poetry() {
    Speedire::new();

    let build_result = PoetryCommandBuilder::new()
    .compile()
    .build();

    assert!(build_result.is_ok());

    Speedire::destroy();
}

#[test]
fn test_execute_kubectl() {
    Speedire::new();

    let deploy_result = KubectlCommandBuilder::new()
    .namespace("speedire")
    .compile()
    .deploy();

    assert!(deploy_result.is_ok());

    Speedire::destroy();
}

#[test]
fn test_pipeline_builder() {

    let build_result = Speedire::new()
    .builder()
    .step(Box::new(
        PoetryCommandBuilder::new()
        .compile()
    ))
    .compile()
    .build();

    let deploy_result = Speedire::new()
    .deployer()
    .step(Box::new(
        KubectlCommandBuilder::new()
        .namespace("speedire")
        .apply("k8s/deployment.yaml")
        .compile()
    ))
    .compile()
    .deploy();

    assert!(build_result.is_ok());
    assert!(deploy_result.is_ok());

    Speedire::destroy();
}
