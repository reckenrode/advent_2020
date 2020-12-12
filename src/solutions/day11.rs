mod waiting_area;

use anyhow::anyhow;
use clap::Clap;
use waiting_area::WaitingArea;

const SEAT: u8 = 'L' as u8;
const PERSON: u8 = '#' as u8;
const FLOOR: u8 = '.' as u8;

fn neighbors(row: usize, column: usize) -> [(usize, usize); 8] {
    [
        (row.wrapping_sub(1),   column.wrapping_sub(1)),
        (row,                   column.wrapping_sub(1)),
        (row.saturating_add(1), column.wrapping_sub(1)),
        (row.wrapping_sub(1),   column),
        (row.saturating_add(1), column),
        (row.wrapping_sub(1),   column.saturating_add(1)),
        (row,                   column.saturating_add(1)),
        (row.saturating_add(1), column.saturating_add(1)),
    ]
}

fn nearby_filter(grid: &mut [u8], width: usize, height: usize) {
    let mut new_grid = Vec::new();
    new_grid.resize(width * height, FLOOR);
    for (row_index, row) in new_grid.chunks_mut(width).enumerate() {
        for column_index in 0 .. width {
            let neighbors = neighbors(row_index, column_index);
            let tally = neighbors.iter()
                .filter(|(r, c)| *r < height && *c < width)
                .fold(0, |sum, (r, c)| {
                    sum + if grid[r * width + c] == PERSON { 1 } else { 0 }
                });
            let current_cell = grid[row_index * width + column_index];
            row[column_index] = match current_cell {
                PERSON if tally >= 4 => SEAT,
                SEAT   if tally == 0 => PERSON,
                _ => current_cell
            }
        }
    }
    grid.copy_from_slice(&new_grid);
}
pub trait Day11Extensions {
    fn wait_until_stable(&mut self);
}

impl Day11Extensions for waiting_area::WaitingArea {
    fn wait_until_stable(&mut self) {
        let mut current_area = self.to_string();
        let mut new_area;
        self.apply_rules(nearby_filter);
        while { new_area = self.to_string(); current_area != new_area } {
            current_area = new_area;
            self.apply_rules(nearby_filter);
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
    use super::{waiting_area::*, *};

    mod nearby_filter {
        use super::*;
        #[test]
        fn an_empty_seat_becomes_occupied_when_no_occupied_seats_are_adjacent() {
            let mut waiting_area = WaitingArea::parse("L.LL\nLLLL").unwrap();
            let expected = "#.##\n####";
            waiting_area.apply_rules(nearby_filter);
            assert_eq!(waiting_area.to_string(), expected);
        }

        #[test]
        fn an_occupied_seat_becomes_empty_when_four_or_more_adjacent_seats_are_occupied() {
            let mut waiting_area = WaitingArea::parse("#.##\n####").unwrap();
            let expected = "#.L#\n#LL#";
            waiting_area.apply_rules(nearby_filter);
            assert_eq!(waiting_area.to_string(), expected);
        }
    }

    mod waiting_process {
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
}
