use crate::toolfs::{BuilderTool, DeployerTool};

use std::error::Error;
use simple_error::bail;

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

pub trait PipelineBuilder {
    fn build(&self) -> Result<bool, Box<dyn Error>>;
}

pub trait PipelineDeployer {
    fn deploy(&self) -> Result<bool, Box<dyn Error>>;
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