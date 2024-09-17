use crate::spec;

pub struct Numeric {
    value: i64,
    spec: spec::Spec,
}

impl Numeric {
    pub fn new(spec: spec::Spec) -> Self {
        Numeric { value: 0, spec }
    }
    pub fn unpack(&mut self, data: &[u8]) -> Result<usize, Box<dyn std::error::Error>> {
        let (raw, bytes_read) = self.spec.unpacker.unpack(data, &self.spec)?;

        self.set_bytes(&raw).map_err(|err| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("falied to set bytes: {}", err),
            ))
        })?;

        Ok(bytes_read)
    }

    fn set_bytes(&mut self, b: &[u8]) -> Result<(), String> {
        if b.is_empty() {
            self.value = 0;
        } else {
            self.value = std::str::from_utf8(b)
                .map_err(|e| e.to_string())?
                .parse::<i64>()
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
