pub trait ValueDecoder {
    fn decode(value: u64, mask: &str) -> u64;
}
