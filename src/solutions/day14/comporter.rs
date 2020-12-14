mod address_decoder;
mod mask;
mod program_statement;
mod value_decoder;

use address_decoder::{AddressDecoder, NullDecoder};
use anyhow::{anyhow, Result};
use mask::Mask;
use program_statement::ProgramStatement;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};
use value_decoder::{MaskedDecoder, ValueDecoder};

const MASK_LEN: usize = 36;
const ZERO_BIT: u8 = '0' as u8;
const ONE_BIT: u8 = '1' as u8;
const ANY_BIT: u8 = 'X' as u8;

pub struct Comporter {
    mask: Mask,
    memory: HashMap<usize, u64>,
    address_decoder: Box<dyn AddressDecoder>,
    value_decoder: Box<dyn ValueDecoder>,
}

impl Comporter {
    pub fn new() -> Comporter {
        Comporter {
            mask: Mask::new(),
            memory: HashMap::new(),
            address_decoder: Box::new(NullDecoder::new()),
            value_decoder: Box::new(MaskedDecoder::new()),
        }
    }

    pub fn exec<'a>(&mut self, src: impl Read) -> Result<()> {
        let reader = BufReader::new(src);
        let lines = reader.lines();
        for line in lines {
            match ProgramStatement::parse(line?.as_ref())? {
                ProgramStatement::Instruction(address, value) => self.set_memory(address, value),
                ProgramStatement::Mask(mask) => self.set_mask(mask)?,
            }
        }
        Ok(())
    }

    pub fn set_mask(&mut self, mask: impl AsRef<str>) -> Result<()> {
        self.mask = Mask::parse(mask)?;
        Ok(())
    }

    pub fn set_memory(&mut self, index: usize, value: u64) {
        let mask = &self.mask.to_string();
        let memory_value = self.value_decoder.decode(value, mask);
        for address in self.address_decoder.decode(index, mask) {
            *self.memory.entry(address).or_insert(memory_value) = memory_value;
        }
    }

    pub fn sum_of_memory(&self) -> u64 {
        self.memory.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn when_it_writes_a_value_it_applies_the_mask_to_the_bits() -> Result<()> {
        let expected_result = 0b000000000000000000000000000001001001;

        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let value = 0b000000000000000000000000000000001011;

        let mut compy = Comporter::new();
        compy.set_mask(mask)?;
        compy.set_memory(0, value);

        Ok(assert_eq!(compy.sum_of_memory(), expected_result))
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

        Ok(assert_eq!(compy.sum_of_memory(), expected_result))
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
    fn it_loads_the_program_and_runs_it() -> Result<()> {
        let program = "\
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
            mem[8] = 11\n\
            mem[7] = 101\n\
            mem[8] = 0";

        let expected_sum = 165;

        let mut compy = Comporter::new();
        compy.exec(Cursor::new(program))?;

        Ok(assert_eq!(compy.sum_of_memory(), expected_sum))
    }

    #[test]
    fn it_supports_changing_masks_in_the_program() -> Result<()> {
        let program = "\
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
            mem[8] = 11\n\
            mem[7] = 101\n\
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX01\n\
            mem[8] = 0";

        let expected_sum = 166;

        let mut compy = Comporter::new();
        compy.exec(Cursor::new(program))?;

        Ok(assert_eq!(compy.sum_of_memory(), expected_sum))
    }
}
