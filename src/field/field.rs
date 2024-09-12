pub trait Field {
    fn unpack(&mut self, data: &[u8]) -> Result<usize, Box<dyn std::error::Error>>;
}
