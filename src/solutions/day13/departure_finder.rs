use std::{cell::RefCell, iter::{Iterator, Peekable, from_fn}};
use is_sorted::IsSorted;

fn multiples_sequence(n: i64) -> RefCell<Peekable<impl Iterator<Item = i64>>> {
    let mut m = n;
    RefCell::new(from_fn(move || {
        let result = Some(m);
        m += n;
        result
    }).peekable())
}

fn earliest_deparature(buses: &[i64], offsets: &[i64]) -> Option<i64> {
    fn current_values<'a>(
        xs: &'a [RefCell<Peekable<impl Iterator<Item = i64>>>]
    ) -> impl Iterator<Item = i64> + 'a {
        xs.iter()
            .map(|cell| {
                let mut it = cell.borrow_mut();
                *it.peek().expect("contains a value")
            })
    }
    fn is_departure_time(xs: impl Iterator<Item = i64>, offsets: &[i64]) -> bool {
        let mut xs = xs.peekable();
        if let Some(first) = xs.peek() {
            let first = *first;
            xs.map(|x| x - first)
                .zip(offsets.iter())
                .fold(true, |acc, (lhs, rhs)| {
                    acc && lhs == *rhs
                })
        } else {
            false
        }
    };
    if buses.len() == offsets.len() && buses.len() > 0 {
        let multiples: Vec<RefCell<Peekable<_>>> = buses.iter()
            .map(|x| multiples_sequence(*x))
            .collect();
        while !is_departure_time(current_values(multiples.as_slice()), offsets) {
            if !IsSorted::is_sorted(&mut current_values(multiples.as_slice())) {
                let mut last = &multiples[0];
                for cell in multiples[1..].iter() {
                    let mut stream = cell.borrow_mut();
                    while stream.peek() < last.borrow_mut().peek() {
                        stream.next();
                    }
                    last = cell;
                }
            } else {
                multiples[0].borrow_mut().next();
            }
        }
        let result: Vec<i64> = current_values(multiples.as_slice())
            .collect();
        Some(result[0])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_the_earliest_timestamp_example_1() {
        let expected_timestamp = Some(3417);
        let buses = [ 17, 13, 19 ];
        let offsets = [ 0, 2, 3 ];
        assert_eq!(earliest_deparature(&buses, &offsets), expected_timestamp)
    }

    #[test]
    fn it_finds_the_earliest_timestamp_example_2() {
        let expected_timestamp = Some(754018);
        let buses = [67, 7, 59, 61];
        let offsets = [ 0, 1, 2, 3 ];
        assert_eq!(earliest_deparature(&buses, &offsets), expected_timestamp)
    }

    #[test]
    fn it_finds_the_earliest_timestamp_example_3() {
        let expected_timestamp = Some(779210);
        let buses = [67, 7, 59, 61];
        let offsets = [ 0, 2, 3, 4 ];
        assert_eq!(earliest_deparature(&buses, &offsets), expected_timestamp)
    }

    #[test]
    fn it_finds_the_earliest_timestamp_example_4() {
        let expected_timestamp = Some(1261476);
        let buses = [67, 7, 59, 61];
        let offsets = [ 0, 1, 3, 4 ];
        assert_eq!(earliest_deparature(&buses, &offsets), expected_timestamp)
    }

    #[test]
    fn it_finds_the_earliest_timestamp_example_5() {
        let expected_timestamp = Some(1202161486);
        let buses = [ 1789, 37, 47, 1889 ];
        let offsets = [ 0, 1, 2, 3 ];
        assert_eq!(earliest_deparature(&buses, &offsets), expected_timestamp)
    }
}
