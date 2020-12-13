use num::{Integer, One};

pub trait NextMultiple {
    fn next_multiple(&self, other: Self) -> Self;
}

impl<T: Integer + One + Copy> NextMultiple for T {
    fn next_multiple(&self, other: Self) -> Self {
        let prev_dividend = *self / other;
        (prev_dividend + Self::one()) * other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_the_next_integer_multiple() {
        let expected_multiple = 945;
        let num = 7;
        let my_time = 939;
        assert_eq!(my_time.next_multiple(num), expected_multiple)
    }
}
