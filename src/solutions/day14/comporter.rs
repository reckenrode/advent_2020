use anyhow::{anyhow, Result};
use std::collections::HashMap;

const MASK_LEN: usize = 36;
const ZERO_BIT: u8 = '0' as u8;
const ONE_BIT: u8 = '1' as u8;
const ANY_BIT: u8 = 'X' as u8;

pub struct Comporter {
    and_mask: u64,
    or_mask: u64,
    memory: HashMap<usize, u64>,
}

impl Comporter {
    pub fn new() -> Comporter {
        Comporter {
            and_mask: u64::MAX,
            or_mask: 0,
            memory: HashMap::new(),
        }
    }

    pub fn set_mask(&mut self, mask: impl AsRef<str>) -> Result<()> {
        let mask = mask.as_ref();
        if mask.len() != MASK_LEN {
            Err(anyhow!(
                "mask length is invalid ({} not {})",
                mask.len(),
                MASK_LEN,
            ))
        } else {
            let (and_mask, or_mask) = Self::parse_masks(mask)?;
            self.and_mask = and_mask;
            self.or_mask = or_mask;
            Ok(())
        }
    }

    pub fn memory(&self, index: usize) -> u64 {
        *self.memory.get(&index).unwrap_or(&0)
    }

    pub fn set_memory(&mut self, index: usize, value: u64) {
        let memory_value = value & self.and_mask | self.or_mask;
        *self.memory.entry(index).or_insert(memory_value) = memory_value;
    }

    pub fn sum_of_memory(&self) -> u64 {
        self.memory.values().sum()
    }

    fn parse_masks(mask: &str) -> Result<(u64, u64)> {
        mask.bytes()
            .try_fold((0, 0), |(and_mask, or_mask), bit| match bit {
                ANY_BIT => Ok((and_mask << 1 | 1, or_mask << 1)),
                ONE_BIT => Ok((and_mask << 1 | 1, or_mask << 1 | 1)),
                ZERO_BIT => Ok((and_mask << 1, or_mask << 1)),
                _ => Err(anyhow!("invalid character encountered in mask")),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_it_writes_a_value_it_applies_the_mask_to_the_bits() -> Result<()> {
        let expected_result = 0b000000000000000000000000000001001001;

        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let value = 0b000000000000000000000000000000001011;

        let mut compy = Comporter::new();
        compy.set_mask(mask)?;
        compy.set_memory(0, value);

        Ok(assert_eq!(compy.memory(0), expected_result))
    }

    #[test]
    fn when_it_writes_a_value_it_overwrites_the_old_value() -> Result<()> {
        let expected_result = 0b000000000000000000000000000001000000;

        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let values = [
            0b000000000000000000000000000001000000,
            0b000000000000000000000000000000000000,
        ];

        let mut compy = Comporter::new();
        compy.set_mask(mask)?;
        for value in values.iter() {
            compy.set_memory(8, *value);
        }

        Ok(assert_eq!(compy.memory(8), expected_result))
    }

    #[test]
    fn sum_is_the_total_of_all_memory_contents() -> Result<()> {
        let expected_sum = 165;

        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let program = [
            (8, 11),
            (7, 101),
            (8, 0),
        ];

        let mut compy = Comporter::new();
        compy.set_mask(mask)?;

        for (address, value) in program.iter() {
            compy.set_memory(*address, *value);
        }

        Ok(assert_eq!(compy.sum_of_memory(), expected_sum))
    }
}
