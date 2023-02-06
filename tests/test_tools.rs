use speedire::kubectl_setup::{KubectlCommandBuilder};
use speedire::toolfs;
use speedire::poetry_setup::{PoetryCommandBuilder};
use std::{path::Path, fs};
use speedire::toolfs::{BuilderTool, DeployerTool};
use speedire::pipelines::{BuildPipelineBuilder, DeployPipelineBuilder};
use speedire::pipelines::{PipelineBuilder, PipelineDeployer};

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

    let build_result = PoetryCommandBuilder::new()
    .compile()
    .build();

    assert!(build_result.is_ok());

    toolfs::cleanup().unwrap();
}

#[test]
fn test_execute_kubectl() {
    toolfs::initialize().unwrap();

    let deploy_result = KubectlCommandBuilder::new()
    .namespace("speedire")
    .compile()
    .deploy();

    assert!(deploy_result.is_ok());

    toolfs::cleanup().unwrap();
}

#[test]
fn test_pipeline_builder() {
    toolfs::initialize().unwrap();

    let poetry_command = PoetryCommandBuilder::new()
    .compile();
    
    let kubectl_command = KubectlCommandBuilder::new()
    .namespace("speedire")
    .apply("k8s/deployment.yaml")
    .compile();

    let pipeline_builder = BuildPipelineBuilder::new()
    .step(Box::new(poetry_command))
    .compile();

    let pipeline_deployer = DeployPipelineBuilder::new()
    .step(Box::new(kubectl_command))
    .compile();

    let build_result = pipeline_builder.build();
    let deploy_result = pipeline_deployer.deploy();

    assert!(build_result.is_ok());
    assert!(deploy_result.is_ok());
}
