use super::{super::mask::Mask, ValueDecoder};

pub struct NullDecoder {}

impl NullDecoder {
    pub fn new() -> NullDecoder {
        NullDecoder {}
    }
}

impl ValueDecoder for NullDecoder {
    fn decode(&self, value: u64, _mask: &Mask) -> u64 {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn passes_through_value_unchanged(value: u64) {
            let expected_value = value;
            let mask = Mask::new();
            let decoder = NullDecoder::new();
            prop_assert_eq!(decoder.decode(value, &mask), expected_value)
        }
    }
}
