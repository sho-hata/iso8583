use crate::decode;
use crate::pad;
use crate::prefix;

pub struct Spec {
    // Length defines the maximum length of the field value (bytes,
    // characters, digits or hex digits), for both fixed and variable
    // lengths. You should use appropriate field types corresponding to the
    // length of the field you're defining, e.g. Numeric, String, Binary
    // etc. For Hex fields, the length is defined in terms of the number of
    // bytes, while the value of the field is hex string.
    length: usize,
    // Tag sets the tag specification. Only applicable to composite field
    // types.
    // tag: Option<TagSpec>,
    // Description of what data the field holds.
    description: String,
    // Enc defines the encoder used to marshal and unmarshal the field.
    // Only applicable to primitive field types e.g. numerics, strings,
    // binary etc
    enc: Box<dyn decode::Decoder>,
    // Pref defines the prefixer of the field used to encode and decode the
    // length of the field.
    pref: Box<dyn prefix::Prefixer>,
    // Pad sets the padding direction and type of the field.
    pad: Box<dyn pad::Padder>,

    pub unpacker: DefaultUnpacker,
}

pub struct DefaultUnpacker;

impl DefaultUnpacker {
    pub fn unpack(
        &self,
        packed_field_value: &[u8],
        spec: &Spec,
    ) -> Result<(Vec<u8>, usize), Box<dyn std::error::Error>> {
        // decode the length
        let (value_length, pref_bytes) = spec
            .pref
            .decode_length(spec.length, packed_field_value)
            .map_err(|e| format!("failed to decode length: {}", e))?;

        // decode the value
        let (value, read) = spec
            .enc
            .decode(&packed_field_value[pref_bytes..], value_length)
            .map_err(|e| format!("failed to decode content: {}", e))?;

        Ok((value, read + pref_bytes))
    }
}
