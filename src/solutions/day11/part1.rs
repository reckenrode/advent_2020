use super::waiting_area::*;

pub (in crate::solutions::day11) fn run_filter(
    grid: &mut [u8], width: usize, height: usize,
    neighbors: impl Fn(&[u8], usize, usize) -> [(usize, usize); 8],
    evaluate_tally: impl Fn(i32, u8) -> u8) {
        let mut new_grid = Vec::new();
        new_grid.resize(width * height, FLOOR);
        for (row_index, row) in new_grid.chunks_mut(width).enumerate() {
            for column_index in 0 .. width {
                let neighbors = neighbors(grid, row_index, column_index);
                let tally = neighbors.iter()
                    .filter(|(r, c)| *r < height && *c < width)
                    .fold(0, |sum, (r, c)| {
                        sum + if grid[r * width + c] == PERSON { 1 } else { 0 }
                    });
                let current_cell = grid[row_index * width + column_index];
                row[column_index] = evaluate_tally(tally, current_cell)
            }
        }
        grid.copy_from_slice(&new_grid);
}

pub fn nearby_filter(grid: &mut [u8], width: usize, height: usize) {
    run_filter(grid, width, height, neighbors, evaluate_tally)
}

fn evaluate_tally(tally: i32, current_cell: u8) -> u8 {
    match current_cell {
        PERSON if tally >= 4 => SEAT,
        SEAT   if tally == 0 => PERSON,
        _ => current_cell
    }
}

fn neighbors(_grid: &[u8], row: usize, column: usize) -> [(usize, usize); 8] {
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

#[cfg(test)]
mod tests {
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
