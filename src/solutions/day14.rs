mod comporter;

use anyhow::Result;
use clap::Clap;

#[derive(Clap)]
pub struct Solution {
    input: std::path::PathBuf,
}

impl Solution {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let program = std::fs::File::open(&self.input)?;
        let mut compy = comporter::Comporter::new();
        compy.exec(program)?;
        println!("The sum of all values in memory is {}.", compy.sum_of_memory());
        Ok(())
    }
}
