mod masked_decoder;

use super::mask::Mask;

pub use masked_decoder::MaskedDecoder;

pub trait ValueDecoder {
    fn decode(&self, value: u64, mask: &Mask) -> u64;
}
