mod null_decoder;

pub use null_decoder::NullDecoder;

pub trait AddressDecoder {
    fn decode(&self, target: usize, mask: &str) -> Vec<usize>;
}
