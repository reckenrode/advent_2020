use super::waiting_area::*;

pub fn nearby_filter(grid: &mut [u8], width: usize, height: usize) {
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
