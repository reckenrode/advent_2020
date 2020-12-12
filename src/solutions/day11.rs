mod part1;
mod part2;
mod waiting_area;

use anyhow::anyhow;
use clap::Clap;
use waiting_area::WaitingArea;

pub trait Day11Extensions {
    fn wait_until_stable(&mut self);
}

impl Day11Extensions for WaitingArea {
    fn wait_until_stable(&mut self) {
        let mut current_area = self.to_string();
        let mut new_area;
        self.apply_rules(part1::nearby_filter);
        while { new_area = self.to_string(); current_area != new_area } {
            current_area = new_area;
            self.apply_rules(part1::nearby_filter);
        }
    }
}

#[derive(Clap)]
pub struct Solution {
    input: std::path::PathBuf,
}

impl Solution {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(&self.input)?;
        let mut room = WaitingArea::parse(&data)
            .ok_or(anyhow!("error parsing room data in the input file"))?;
        room.wait_until_stable();
        let contents = room.to_string();
        let occupied_seats = contents.chars()
            .filter(|ch| *ch == '#')
            .count();
        println!("There are {} occupied seats.", occupied_seats);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_waiting_area_stabilizes() {
        let initial_room = "\
            L.LL.LL.LL\n\
            LLLLLLL.LL\n\
            L.L.L..L..\n\
            LLLL.LL.LL\n\
            L.LL.LL.LL\n\
            L.LLLLL.LL\n\
            ..L.L.....\n\
            LLLLLLLLLL\n\
            L.LLLLLL.L\n\
            L.LLLLL.LL";
        let expected_area_contents = "\
            #.#L.L#.##\n\
            #LLL#LL.L#\n\
            L.#.L..#..\n\
            #L##.##.L#\n\
            #.#L.LL.LL\n\
            #.#L#L#.##\n\
            ..L.L.....\n\
            #L#L##L#L#\n\
            #.LLLLLL.L\n\
            #.#L#L#.##";
        let mut waiting_area = WaitingArea::parse(initial_room).unwrap();
        waiting_area.wait_until_stable();
        assert_eq!(waiting_area.to_string(), expected_area_contents);
    }
}
