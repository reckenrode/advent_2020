use num::Integer;
use std::iter::Iterator;

pub fn earliest_departure(buses: &[u64], offsets: &[usize]) -> Option<u64> {
    if buses.len() == offsets.len() && buses.len() > 0 {
        let mut result = buses[0];
        let mut increment = buses[0];
        for (bus, offset) in buses.iter().skip(1).zip(offsets.iter().skip(1)) {
            while (result + *offset as u64) % bus != 0 {
                result += increment;
            }
            increment = increment.lcm(bus);
        }
        Some(result)
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
        assert_eq!(earliest_departure(&buses, &offsets), expected_timestamp)
    }

    #[test]
    fn it_finds_the_earliest_timestamp_example_2() {
        let expected_timestamp = Some(754018);
        let buses = [67, 7, 59, 61];
        let offsets = [ 0, 1, 2, 3 ];
        assert_eq!(earliest_departure(&buses, &offsets), expected_timestamp)
    }

    #[test]
    fn it_finds_the_earliest_timestamp_example_3() {
        let expected_timestamp = Some(779210);
        let buses = [67, 7, 59, 61];
        let offsets = [ 0, 2, 3, 4 ];
        assert_eq!(earliest_departure(&buses, &offsets), expected_timestamp)
    }

    #[test]
    fn it_finds_the_earliest_timestamp_example_4() {
        let expected_timestamp = Some(1261476);
        let buses = [67, 7, 59, 61];
        let offsets = [ 0, 1, 3, 4 ];
        assert_eq!(earliest_departure(&buses, &offsets), expected_timestamp)
    }

    #[test]
    fn it_finds_the_earliest_timestamp_example_5() {
        let expected_timestamp = Some(1202161486);
        let buses = [ 1789, 37, 47, 1889 ];
        let offsets = [ 0, 1, 2, 3 ];
        assert_eq!(earliest_departure(&buses, &offsets), expected_timestamp)
    }
}
