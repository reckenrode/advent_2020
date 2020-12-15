use anyhow::{anyhow, Result};
use std::convert::{TryFrom, TryInto};

pub const MASK_LEN: usize = 36;

const ZERO_BIT: u8 = '0' as u8;
const ONE_BIT: u8 = '1' as u8;
const ANY_BIT: u8 = 'X' as u8;

#[derive(Debug)]
pub struct Mask {
    raw_mask: [Bit; 36],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bit {
    Any,
    One,
    Zero,
}

impl Mask {
    pub fn new() -> Mask {
        Mask {
            raw_mask: [Bit::Any; 36],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Bit> {
        self.raw_mask.iter()
    }

    pub fn parse(input: impl AsRef<str>) -> Result<Mask> {
        let input = input.as_ref();
        if input.len() != MASK_LEN {
            Err(anyhow!(
                "invalid mask length (got {} instead of {})",
                input.len(),
                MASK_LEN,
            ))
        } else {
            let mut mask = Mask {
                raw_mask: [Bit::Any; 36],
            };
            for (offset, bit) in input.bytes().enumerate() {
                match bit {
                    ZERO_BIT => mask.raw_mask[offset] = Bit::Zero,
                    ONE_BIT => mask.raw_mask[offset] = Bit::One,
                    ANY_BIT => mask.raw_mask[offset] = Bit::Any,
                    _ => Err(anyhow!(
                        "encountered invalid character in mask at offset {}",
                        offset
                    ))?,
                }
            }
            Ok(mask)
        }
    }
}

impl std::fmt::Display for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bit in self.raw_mask.iter() {
            let raw_char = match bit {
                Bit::Zero => "0",
                Bit::One => "1",
                Bit::Any => "X",
            };
            write!(f, "{}", raw_char)?;
        }
        Ok(())
    }
}

impl TryFrom<Vec<Bit>> for Mask {
    type Error = anyhow::Error;

    fn try_from(value: Vec<Bit>) -> Result<Self, Self::Error> {
        if value.len() != MASK_LEN {
            Err(anyhow!(
                "invalid mask length (got {} instead of {})",
                value.len(),
                MASK_LEN,
            ))
        } else {
            let mut mask = Mask::new();
            for (idx, bit) in value.iter().enumerate() {
                mask.raw_mask[idx] = *bit;
            }
            Ok(mask)
        }
    }
}

impl TryInto<usize> for Mask {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<usize, Self::Error> {
        let mut result = 0;
        for bit in self.raw_mask.iter() {
            match bit {
                Bit::Zero => result <<= 1,
                Bit::One => result = result << 1 | 1,
                Bit::Any => Err(anyhow!("can’t convert a mask with any bits to a usize"))?,
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_a_mask_successfully() -> Result<()> {
        let expected_mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        Ok(assert_eq!(
            Mask::parse(expected_mask)?.to_string(),
            expected_mask
        ))
    }

    #[test]
    fn creates_a_default_mask_with_all_any_bits() {
        let expected_mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
        assert_eq!(Mask::new().to_string(), expected_mask)
    }

    #[test]
    fn rejects_masks_that_are_too_long() {
        let expected_result = anyhow!("invalid mask length (got 37 instead of 36)");
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0XX";
        let result = Mask::parse(mask).unwrap_err();
        assert_eq!(result.to_string(), expected_result.to_string())
    }

    #[test]
    fn rejects_masks_that_are_too_short() {
        let expected_result = anyhow!("invalid mask length (got 35 instead of 36)");
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0";
        let result = Mask::parse(mask).unwrap_err();
        assert_eq!(result.to_string(), expected_result.to_string())
    }

    #[test]
    fn rejects_masks_that_have_invalid_elements() {
        let expected_result = anyhow!("encountered invalid character in mask at offset 22");
        let mask = "XXXXXXXXXXXXXXXXXXXXXX💩XXXX1XXXX0";
        let result = Mask::parse(mask).unwrap_err();
        assert_eq!(result.to_string(), expected_result.to_string())
    }

    #[test]
    fn can_iterate_over_its_bits() -> Result<()> {
        let expected_result = vec![
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::One,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Zero,
            Bit::Any,
        ];
        let mask = Mask::parse("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")?;
        let result: Vec<Bit> = mask.iter().copied().collect();
        Ok(assert_eq!(result, expected_result))
    }

    #[test]
    fn converts_from_a_vec_of_bits() -> Result<()> {
        let expected_result = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let value = vec![
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::One,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Zero,
            Bit::Any,
        ];
        let result: Mask = value.try_into()?;
        Ok(assert_eq!(result.to_string(), expected_result))
    }

    #[test]
    fn rejects_long_bit_vecs() {
        let expected_result = anyhow!("invalid mask length (got 37 instead of 36)");
        let value = vec![
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::One,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Zero,
            Bit::Any,
        ];
        let result = Mask::try_from(value).unwrap_err();
        assert_eq!(result.to_string(), expected_result.to_string())
    }

    #[test]
    fn rejects_short_bit_vecs() {
        let expected_result = anyhow!("invalid mask length (got 35 instead of 36)");
        let value = vec![
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::One,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Any,
            Bit::Zero,
            Bit::Any,
        ];
        let result = Mask::try_from(value).unwrap_err();
        assert_eq!(result.to_string(), expected_result.to_string())
    }
}
