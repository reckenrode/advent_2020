mod masked_decoder;

pub use masked_decoder::MaskedDecoder;

pub trait ValueDecoder {
    fn decode(&self, value: u64, mask: &str) -> u64;
}
