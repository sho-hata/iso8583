pub trait Decoder {
    // Decode decodes data into bytes (ASCII, characters, digits,
    // etc.). It returns the bytes representing the decoded data, the
    // number of bytes read from the input, and any errorh
    fn decode(
        &self,
        data: &[u8],
        length: usize,
    ) -> Result<(Vec<u8>, usize), Box<dyn std::error::Error>>;
}
