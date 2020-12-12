use super::waiting_area::*;
use super::part1::run_filter;

pub fn line_of_sight_filter(grid: &mut [u8], width: usize, height: usize) {
    run_filter(grid, width, height, |grid, row, column|
        {
            [
                cast_ray(grid, row, column, width, height, -1, -1),
                cast_ray(grid, row, column, width, height,  0, -1),
                cast_ray(grid, row, column, width, height,  1, -1),
                cast_ray(grid, row, column, width, height, -1,  0),
                cast_ray(grid, row, column, width, height,  1,  0),
                cast_ray(grid, row, column, width, height, -1,  1),
                cast_ray(grid, row, column, width, height,  0,  1),
                cast_ray(grid, row, column, width, height,  1,  1),
            ]
        }, evaluate_tally)
}

fn cast_ray(grid: &[u8], row: usize, column: usize, width: usize, height: usize, dx: isize, dy: isize) -> (usize, usize) {
    let dx_dir = dx >= 0; let dx = dx.abs() as usize;
    let dy_dir = dy >= 0; let dy = dy.abs() as usize;
    let mut r = if dx_dir { row.saturating_add(dx) } else { row.wrapping_sub(dx) };
    let mut c = if dy_dir { column.saturating_add(dy) } else { column.wrapping_sub(dy) };
    while r < height && c < width && grid[r * width + c] == FLOOR {
        r = if dx_dir { r.saturating_add(dx) } else { r.wrapping_sub(dx) };
        c = if dy_dir { c.saturating_add(dy) } else { c.wrapping_sub(dy) };
    }
    (r, c)
}

fn evaluate_tally(tally: i32, current_cell: u8) -> u8 {
    match current_cell {
        PERSON if tally >= 5 => SEAT,
        SEAT   if tally == 0 => PERSON,
        _ => current_cell
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_seat_becomes_occupied_when_only_empty_seats_can_be_seen() {
        let mut waiting_area = WaitingArea::parse("L.LL\nLLLL").unwrap();
        let expected = "#.##\n####";
        waiting_area.apply_rules(line_of_sight_filter);
        assert_eq!(waiting_area.to_string(), expected);
    }

    #[test]
    fn an_occupied_seat_becomes_empty_when_five_or_more_visible_seats_are_occupied() {
        let mut waiting_area = WaitingArea::parse("####\n####").unwrap();
        let expected = "#LL#\n#LL#";
        waiting_area.apply_rules(line_of_sight_filter);
        assert_eq!(waiting_area.to_string(), expected);
    }
}
