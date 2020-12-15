use std::{collections::HashMap, iter::from_fn};

fn counting_game_iter(input: &[i32]) -> impl Iterator<Item = i32> {
    let mut initial_numbers = input.to_vec();
    initial_numbers.reverse();

    let mut game_state: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut last_number = 0;
    let mut turn_number = 0;

    from_fn(move || {
        if let Some(number) = initial_numbers.pop() {
            last_number = number;
        } else if let Some((last_spoken, prev_spoken)) = game_state.get(&last_number) {
            last_number = last_spoken - prev_spoken;
        } else {
            last_number = 0;
        }
        game_state
            .entry(last_number)
            .and_modify(|e| *e = (turn_number, e.0))
            .or_insert((turn_number, turn_number));
        turn_number += 1;
        Some(last_number)
    })
}

#[cfg(test)]
mod tests {
    use super::counting_game_iter;

    #[test]
    fn the_sequence_matches_the_example() {
        let input = [0, 3, 6];
        let expected_sequence = [0, 3, 6, 0, 3, 3, 1, 0, 4, 0];
        let result: Vec<i32> = counting_game_iter(&input).take(10).collect();
        assert_eq!(result, expected_sequence);
    }

    #[test]
    fn handles_new_numbers() {
        let input = [1, 3, 6];
        let expected_sequence = [1, 3, 6, 0, 0, 1, 5, 0, 3, 7];
        let result: Vec<i32> = counting_game_iter(&input).take(10).collect();
        assert_eq!(result, expected_sequence);
    }
}
