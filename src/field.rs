use std::collections::HashMap;
use std::error::Error;

const MTIIDX: i32 = 0;

pub trait Field {
    fn unpack(&mut self, data: &[u8]) -> Result<usize, Box<dyn std::error::Error>>;
    fn set_bytes(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Message {
    fields: HashMap<i32, Box<dyn Field>>,
}

impl Message {
    fn unpack(&mut self, src: &[u8]) -> Result<String, Box<dyn Error>> {
        if let Some(field) = self.fields.get_mut(&MTIIDX) {
            match field.unpack(src) {
                Ok(read) => Ok(format!("read {} bytes", read)),
                Err(err) => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("failed to unpack MTI: {}", err),
                ))),
            }
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Field not found",
            )))
        }
    }
}
