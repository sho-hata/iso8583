pub trait Padder {
    fn pad(&self, data: &[u8], length: usize) -> Option<&[u8]>;
}
