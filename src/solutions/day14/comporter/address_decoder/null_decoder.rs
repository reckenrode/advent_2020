use super::{super::mask::Mask, AddressDecoder};

pub struct NullDecoder {}

impl NullDecoder {
    pub fn new() -> NullDecoder {
        NullDecoder {}
    }
}

impl AddressDecoder for NullDecoder {
    fn decode(&self, target: usize, _mask: &Mask) -> Vec<usize> {
        vec![target]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn passes_through_address_unchanged(address: usize) {
            let expected_address = vec![ address ];
            let mask = Mask::new();
            let decoder = NullDecoder::new();
            prop_assert_eq!(decoder.decode(address, &mask), expected_address)
        }
    }
}
