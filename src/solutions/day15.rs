mod counting_game;

use anyhow::anyhow;
use clap::Clap;
use std::error::Error;

#[derive(Clap)]
pub struct Solution {
    input: std::path::PathBuf,
    #[clap(default_value = "2020")]
    nth: usize,
}

impl Solution {
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let input: Result<Vec<i32>, _> = std::fs::read_to_string(self.input)?
            .strip_suffix("\n").ok_or(anyhow!("invalid input format"))?
            .split(",")
            .map(|x| x.parse())
            .collect();
        let mut sequence = counting_game::counting_game_iter(&input?);
        let target_number = sequence
            .nth(self.nth - 1)
            .ok_or(anyhow!("something has gone terribly wrong. :("))?;
        println!("#{} in the game is {}.", self.nth, target_number);
        Ok(())
    }
}
