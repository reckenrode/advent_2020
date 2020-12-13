use clap::Clap;

mod part1;
mod ship;

use ship::{Action, Ship};

#[derive(Clap)]
pub struct Solution {
    input: std::path::PathBuf,
}

impl Solution {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(&self.input)?;
        let lines: Result<Vec<Action>, anyhow::Error> = data.lines()
            .map(Action::parse)
            .collect();
        let mut ship = Ship::new();
        for line in lines? {
            ship.act(line);
        }
        let distance = part1::manhattan_distance(ship.position());
        println!("The manhattan distance is {}.", distance);
        Ok(())
    }
}
