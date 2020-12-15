use super::super::mask::{Bit, Mask, MASK_LEN};
use super::AddressDecoder;
use anyhow::Result;
use bitvec::prelude::*;
use std::convert::{TryFrom, TryInto};

pub struct FloatingDecoder {}

impl FloatingDecoder {
    pub fn new() -> FloatingDecoder {
        FloatingDecoder {}
    }
}

impl AddressDecoder for FloatingDecoder {
    fn decode(&self, address: usize, mask: &Mask) -> Vec<usize> {
        let address_bits = address.view_bits::<Msb0>();
        let combined_mask: Vec<Bit> = address_bits
            .into_iter()
            .skip(address_bits.len() - MASK_LEN)
            .zip(mask.iter())
            .map(|(addr_bit, mask_bit)| match mask_bit {
                Bit::zero => {
                    if *addr_bit {
                        Bit::one
                    } else {
                        Bit::zero
                    }
                }
                x => *x,
            })
            .collect();
        let combined_mask: Mask = combined_mask.try_into().expect("valid mask");
        masks_for_floating_bits(&combined_mask)
            .into_iter()
            .map(|m| m.try_into().expect("valid u64"))
            .collect()
    }
}

fn masks_for_floating_bits(mask: &Mask) -> Vec<Mask> {
    fn masks_for_floating_bits_rec(bits: &[Bit], offset: usize, result: &mut Vec<Vec<Bit>>) {
        if offset == bits.len() {
            result.push(bits.to_vec())
        } else if bits[offset] == Bit::any {
            let mut one_mask = bits.to_vec();
            one_mask[offset] = Bit::one;
            let mut zero_mask = bits.to_vec();
            zero_mask[offset] = Bit::zero;
            masks_for_floating_bits_rec(one_mask.as_slice(), offset + 1, result);
            masks_for_floating_bits_rec(zero_mask.as_slice(), offset + 1, result);
        } else {
            masks_for_floating_bits_rec(bits, offset + 1, result)
        }
    }
    let mask_vec: Vec<Bit> = mask.iter().copied().collect();
    let mut masks = Vec::new();
    masks_for_floating_bits_rec(mask_vec.as_slice(), 0, &mut masks);
    let masks: Result<Vec<Mask>> = masks.into_iter().map(|v| Mask::try_from(v)).collect();
    masks.expect("valid bit mask")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_all_addresses_for_the_floating_bits() -> Result<()> {
        let mut expected_addresses = vec![
            0b000000000000000000000000000000011010,
            0b000000000000000000000000000000011011,
            0b000000000000000000000000000000111010,
            0b000000000000000000000000000000111011,
        ];
        expected_addresses.sort();
        let mask = Mask::parse("000000000000000000000000000000X1001X")?;
        let address = 42;
        let decoder = FloatingDecoder::new();
        let mut result = decoder.decode(address, &mask);
        result.sort();
        Ok(assert_eq!(result, expected_addresses))
    }
}
