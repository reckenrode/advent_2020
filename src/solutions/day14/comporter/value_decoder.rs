mod masked_decoder;
mod null_decoder;

use super::mask::Mask;

pub use masked_decoder::MaskedDecoder;
pub use null_decoder::NullDecoder;

pub trait ValueDecoder {
    fn decode(&self, value: u64, mask: &Mask) -> u64;
}
