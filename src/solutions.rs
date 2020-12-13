use clap::Clap;
use std::error::Error;

mod day11;
mod day12;

#[derive(Clap)]
pub enum Solution {
    Day11(day11::Solution),
    Day12(day12::Solution),
}

impl Solution {
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        match self {
            Solution::Day11(solution) => solution.run(),
            Solution::Day12(solution) => solution.run(),
        }
    }
}
