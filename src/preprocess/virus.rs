use anyhow::Result;
use derive_builder::Builder;

// use std::io::Write;
use std::path::PathBuf;




#[derive(Builder, Clone)]
pub struct Caller {
    genome: PathBuf,
    reads: Vec<PathBuf>,
    // output: Option<PathBuf>,
}

impl Caller {
    pub fn call(&self) -> Result<()> {
        Ok(())
    }
}
