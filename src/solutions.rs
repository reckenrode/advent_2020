use clap::Clap;

mod day11;

#[derive(Clap)]
pub enum Solution {
    Day11
}

impl Solution {
    pub fn run(self) {
        match self {
            Solution::Day11 => println!("got day 11")
        }
    }
}
