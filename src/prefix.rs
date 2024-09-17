pub trait Prefixer {
    // Returns the size of the field (number of characters, HEX-digits, bytes)
    // as well as the number of bytes read to decode the length
    fn decode_length(
        &self,
        max_len: usize,
        data: &[u8],
    ) -> Result<(usize, usize), Box<dyn std::error::Error>>;
}
