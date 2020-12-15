use super::{
    address_decoder::{AddressDecoder, FloatingDecoder, NullDecoder as AddressNullDecoder},
    value_decoder::{MaskedDecoder, NullDecoder as ValueNullDecoder, ValueDecoder}
};
use clap::Clap;

#[derive(Clap)]
pub enum Version {
    One,
    Two,
}

impl Version {
    pub fn decoders(&self) -> (Box<dyn AddressDecoder>, Box<dyn ValueDecoder>) {
        match self {
            Self::One => (Box::new(AddressNullDecoder::new()), Box::new(MaskedDecoder::new())),
            Self::Two => (Box::new(FloatingDecoder::new()), Box::new(ValueNullDecoder::new())),
        }
    }
}
