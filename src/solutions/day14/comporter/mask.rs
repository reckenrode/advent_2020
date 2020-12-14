use anyhow::{anyhow, Result};

const MASK_LEN: usize = 36;
const ZERO_BIT: u8 = '0' as u8;
const ONE_BIT: u8 = '1' as u8;
const ANY_BIT: u8 = 'X' as u8;

#[derive(Debug)]
pub struct Mask {
    raw_mask: [Bit; 36],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bit {
    any,
    one,
    zero,
}

impl Mask {
    pub fn new() -> Mask {
        Mask {
            raw_mask: [Bit::any; 36],
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
                raw_mask: [Bit::any; 36],
            };
            for (offset, bit) in input.bytes().enumerate() {
                match bit {
                    ZERO_BIT => mask.raw_mask[offset] = Bit::zero,
                    ONE_BIT => mask.raw_mask[offset] = Bit::one,
                    ANY_BIT => mask.raw_mask[offset] = Bit::any,
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
                Bit::zero => "0",
                Bit::one => "1",
                Bit::any => "X",
            };
            write!(f, "{}", raw_char)?;
        }
        Ok(())
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
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::one,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::any,
            Bit::zero,
            Bit::any,
        ];
        let mask = Mask::parse("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")?;
        let result: Vec<Bit> = mask.iter().copied().collect();
        Ok(assert_eq!(result, expected_result))
    }
}
