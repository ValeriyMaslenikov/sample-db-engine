use bincode::{
    config::{ Configuration, NoLimit, Fixint, LittleEndian, SkipFixedArrayLength },
    Encode,
    Decode,
    error::DecodeError,
};

type EncodedBytes = usize;

const BINCODE_CONFIG: Configuration<
    LittleEndian,
    Fixint,
    SkipFixedArrayLength,
    NoLimit
> = bincode::config::standard().with_fixed_int_encoding().skip_fixed_array_length();

pub fn encode<E: Encode>(value: &E, buffer: &mut [u8]) -> EncodedBytes {
    bincode::encode_into_slice(value, buffer, BINCODE_CONFIG).unwrap()
}

pub fn decode_unwrapped<E: Decode>(buffer: &[u8]) -> E {
    bincode::decode_from_slice(buffer, BINCODE_CONFIG).unwrap().0
}

pub fn decode<E: Decode>(buffer: &[u8]) -> Result<E, DecodeError> {
    bincode::decode_from_slice(buffer, BINCODE_CONFIG).map(|(res, _)| -> E { res })
}