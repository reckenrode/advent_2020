pub trait AddressDecoder {
    fn decode(&self, target: usize, mask: &str) -> Vec<usize>;
}
