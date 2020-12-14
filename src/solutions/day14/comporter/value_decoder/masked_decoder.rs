use super::ValueDecoder;

const ZERO_BIT: u8 = '0' as u8;
const ONE_BIT: u8 = '1' as u8;
const ANY_BIT: u8 = 'X' as u8;

pub struct MaskedDecoder {}

impl MaskedDecoder {
    pub fn new() -> MaskedDecoder {
        MaskedDecoder {}
    }

    fn parse_masks(mask: &str) -> (u64, u64) {
        mask.bytes()
            .fold((0, 0), |(and_mask, or_mask), bit| match bit {
                ANY_BIT => (and_mask << 1 | 1, or_mask << 1),
                ONE_BIT => (and_mask << 1 | 1, or_mask << 1 | 1),
                ZERO_BIT => (and_mask << 1, or_mask << 1),
                _ => panic!("invalid character encountered in mask"),
            })
    }
}

impl ValueDecoder for MaskedDecoder {
    fn decode(&self, value: u64, mask: &str) -> u64 {
        let (and_mask, or_mask) = Self::parse_masks(mask);
        value & and_mask | or_mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_the_mask_to_the_value() {
        let expected_value = 73;
        let value = 0b000000000000000000000000000000001011;
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let decoder = MaskedDecoder::new();
        assert_eq!(decoder.decode(value, mask), expected_value);
    }
}
