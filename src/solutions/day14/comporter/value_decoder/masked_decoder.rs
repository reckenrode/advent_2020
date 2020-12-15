use super::{
    super::mask::{Bit, Mask},
    ValueDecoder,
};
use anyhow::Result;
use std::iter::Iterator;

const ZERO_BIT: u8 = '0' as u8;
const ONE_BIT: u8 = '1' as u8;
const ANY_BIT: u8 = 'X' as u8;

pub struct MaskedDecoder {}

impl MaskedDecoder {
    pub fn new() -> MaskedDecoder {
        MaskedDecoder {}
    }

    fn parse_masks(mask: &Mask) -> (u64, u64) {
        mask.iter()
            .fold((0, 0), |(and_mask, or_mask), bit| match bit {
                Bit::any => (and_mask << 1 | 1, or_mask << 1),
                Bit::one => (and_mask << 1 | 1, or_mask << 1 | 1),
                Bit::zero => (and_mask << 1, or_mask << 1),
            })
    }
}

impl ValueDecoder for MaskedDecoder {
    fn decode(&self, value: u64, mask: &Mask) -> u64 {
        let (and_mask, or_mask) = Self::parse_masks(mask);
        value & and_mask | or_mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_the_mask_to_the_value() -> Result<()> {
        let expected_value = 73;
        let value = 0b000000000000000000000000000000001011;
        let mask = Mask::parse("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")?;
        let decoder = MaskedDecoder::new();
        Ok(assert_eq!(decoder.decode(value, &mask), expected_value))
    }
}