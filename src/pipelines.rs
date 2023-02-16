use crate::metadata;

use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Output;

use simple_error::bail;

pub struct Speedire {
    // pub build_pipeline: Option<BuildPipeline>,
    // pub deploy_pipeline: Option<DeployPipeline>,
}

impl Speedire {
    pub fn new() -> Speedire {
        initialize()
        .expect("unable to initialize the speedire system");

        return Speedire {
            // build_pipeline: None,
            // deploy_pipeline: None,
        }
    }

    pub fn destroy() {
        cleanup()
        .expect("unable to cleanup speedire system")
    }

    pub fn builder(&self) -> BuildPipelineBuilder {
        BuildPipelineBuilder::new()
    }

    pub fn deployer(&self) -> DeployPipelineBuilder {
        DeployPipelineBuilder::new()
    }

    // pub fn build_pipeline(&mut self, build_pipeline: BuildPipeline) -> &mut Speedire {
    //     self.build_pipeline = Some(build_pipeline);
    //     self
    // }

    // pub fn deploy_pipeline(&mut self, deploy_pipeline: DeployPipeline) -> &Speedire {
    //     self.deploy_pipeline = Some(deploy_pipeline);
    //     self
    // }

    // pub fn build(self) -> Result<bool, Box<dyn Error>> {
    //     match self.build_pipeline {
    //         Some(bp) => bp.build(),
    //         None => Ok(true),
    //     }
    // }

    // pub fn deploy(self) -> Result<bool, Box<dyn Error>> {
    //     match self.deploy_pipeline {
    //         Some(dp) => dp.deploy(),
    //         None => Ok(true),
    //     }
    // }
}

/// Initialize the Spedire tool system by creating the temporary
/// download diretory, the bin directory and the opt directory 
/// for tool executables and versions
/// 
/// # Errors
/// Errors due to any issue working with the file system
fn initialize() -> Result<(), Box<dyn Error>> {
    let tools_home = metadata::get_tools_home()?;

    if !Path::new(&tools_home.tool_tmp_dir).exists() {
        println!("creating spedire tmp dir: {}", &tools_home.tool_tmp_dir);
        fs::create_dir_all(&tools_home.tool_tmp_dir)?;
    }

    if !Path::new(&tools_home.tool_bin_dir).exists() {
        println!("creating spedire bin dir: {}", &tools_home.tool_bin_dir);
        fs::create_dir_all(&tools_home.tool_bin_dir)?;
    }

    if !Path::new(&tools_home.tool_opt_dir).exists() {
        println!("creating spedire opt dir: {}", &tools_home.tool_opt_dir);
        fs::create_dir_all(&tools_home.tool_opt_dir)?;
    }

    return Ok(())
}

/// Cleanup the temporary download directory and the bin directory
/// that are created as part of initializing the Spedire tool.
/// 
/// # Errors
/// Errors due to any issues with the filesystem
fn cleanup() -> Result<(), Box<dyn Error>> {
    let tools_home = metadata::get_tools_home()?;

    if Path::new(&tools_home.tool_home).exists() {
        println!("removing speedire working directory: {}", &tools_home.tool_home);
        fs::remove_dir_all(&tools_home.tool_home)?;
    }

    return Ok(())
}

pub trait PipelineBuilder {
    fn build(&self) -> Result<bool, Box<dyn Error>>;
}

pub trait PipelineDeployer {
    fn deploy(&self) -> Result<bool, Box<dyn Error>>;
}

pub struct BuildPipeline {
    builders: Vec<Box<dyn BuilderTool>>
}

pub struct BuildPipelineBuilder {
    pub builders: Vec<Box<dyn BuilderTool>>
}

impl BuildPipelineBuilder {
    pub fn new() -> BuildPipelineBuilder {
        BuildPipelineBuilder { builders: Vec::new() }
    }

    pub fn step(mut self, builder: Box<dyn BuilderTool>) -> BuildPipelineBuilder {
        self.builders.push(builder);
        self
    } 

    pub fn compile(self) -> BuildPipeline {
        BuildPipeline { builders: self.builders }
    }
}

impl PipelineBuilder for BuildPipeline {
    fn build(&self) -> Result<bool, Box<dyn Error>> {
        for builder in &self.builders {
            match builder.build() {
                Ok(_o) => (),
                Err(e) => bail!("pipeline build step failed {:?}", e),
            }
        };

        Ok(true)
    }
}

pub struct DeployPipeline {
    deployers: Vec<Box<dyn DeployerTool>>
}

pub struct DeployPipelineBuilder {
    pub deployers: Vec<Box<dyn DeployerTool>>
}

impl DeployPipelineBuilder {
    pub fn new() -> DeployPipelineBuilder { 
        DeployPipelineBuilder { deployers: Vec::new() } 
    }

    pub fn step(mut self, deployer: Box<dyn DeployerTool>) -> DeployPipelineBuilder {
        self.deployers.push(deployer);
        self
    }

    pub fn compile(self) -> DeployPipeline {
        DeployPipeline { deployers: self.deployers }
    }
}

impl PipelineDeployer for DeployPipeline {
    fn deploy(&self) -> Result<bool, Box<dyn Error>> {
        for deployer in &self.deployers {
            match deployer.deploy() {
                Ok(()) => (),
                Err(e) => bail!("pipeline deploy step failed {:?}", e),
            }
        };

        Ok(true)
    }
}

pub trait BuilderTool {
    // fn execute_with_args(&self, args: &[&str]) -> Box<dyn Tool>;
    // fn execute(&self, arg: &str) -> Box<dyn Tool>;
    fn build(&self) -> Result<Output, Box<dyn Error>>;
}

pub trait DeployerTool {
    fn deploy(&self) -> Result<(), Box<dyn Error>>;
}
