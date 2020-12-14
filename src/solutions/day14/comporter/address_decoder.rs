mod null_decoder;

use super::mask::Mask;

pub use null_decoder::NullDecoder;

pub trait AddressDecoder {
    fn decode(&self, target: usize, mask: &Mask) -> Vec<usize>;
}
