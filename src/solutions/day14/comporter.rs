use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, space0},
    combinator::{eof, map_res},
    sequence::{delimited, pair, preceded, terminated},
    Finish,
};
use std::{collections::HashMap, error::Error};

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

    pub fn exec<'a>(&mut self, src: &'a str) -> Result<(), Box<dyn Error + 'a>> {
        let mut lines = src.lines();
        let header = lines
            .next()
            .ok_or(anyhow!("expected mask but found something else"))?;
        self.set_mask(Self::parse_header(header)?)?;
        for line in lines {
            let (address, value) = Self::parse_line(line)?;
            self.set_memory(address, value);
        }
        Ok(())
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

    fn parse_header<'a>(line: &'a str) -> Result<&'a str, nom::error::Error<&'a str>> {
        let mask_def = tag("mask");
        let mask_content = take_while1(|s| s == 'X' || s == '1' || s == '0');
        let mut mask_statement = terminated(
            preceded(
                mask_def,
                preceded(delimited(space0, char('='), space0), mask_content),
            ),
            eof,
        );
        let (_, result) = mask_statement(line).finish()?;
        Ok(result)
    }

    fn parse_line<'a>(line: &'a str) -> Result<(usize, u64), nom::error::Error<&'a str>> {
        let mem_ref = delimited(
            char('['),
            map_res(digit1, |s: &str| s.parse::<usize>()),
            char(']'),
        );
        let mem_contents = map_res(digit1, |s: &str| s.parse::<u64>());
        let mut mem_statement = terminated(
            pair(
                preceded(tag("mem"), mem_ref),
                preceded(delimited(space0, char('='), space0), mem_contents),
            ),
            eof,
        );
        let (_, result) = mem_statement(line).finish()?;
        Ok(result)
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

    #[test]
    fn it_loads_the_program_and_runs_it() -> Result<(), Box<dyn Error>> {
        let program = "\
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
            mem[8] = 11\n\
            mem[7] = 101\n\
            mem[8] = 0";

        let expected_sum = 165;

        let mut compy = Comporter::new();
        compy.exec(program)?;

        Ok(assert_eq!(compy.sum_of_memory(), expected_sum))
    }
}
