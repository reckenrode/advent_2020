pub trait ValueDecoder {
    fn decode(&self, value: u64, mask: &str) -> u64;
}
