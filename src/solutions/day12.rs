use clap::Clap;

mod part1;
mod ship;

use ship::{Action, Ship};

#[derive(Clap)]
pub struct Solution {
    input: std::path::PathBuf,
    #[clap(
        long,
        about = "Interpret movement instructions in the input file as manipulating the waypoint \
                instead of the ship"
    )]
    enable_waypoint: bool,
}

impl Solution {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(&self.input)?;
        let lines: Result<Vec<Action>, anyhow::Error> = data.lines()
            .map(|line| Action::parse(line, self.enable_waypoint))
            .collect();
        let mut ship = Ship::new();
        if self.enable_waypoint {
            ship.act(Action::MoveWaypointEast(9));
            ship.act(Action::MoveWaypointNorth(1));
        }
        for line in lines? {
            ship.act(line);
        }
        let distance = part1::manhattan_distance(ship.position());
        println!("The manhattan distance is {}.", distance);
        Ok(())
    }
}
