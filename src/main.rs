use std::collections::HashMap;

struct Message {
    mti: String,
    bitmap: Vec<u8>,
    fields: HashMap<u8, String>,
}

fn main() {
    let input_message = b"020011111111101234567890..."; // dummy
    match parse_message(input_message) {
        Ok(message) => {
            println!("MTI: {}", message.mti);
            println!("bitmap: {:?}", message.bitmap);
            for (field_number, field_value) in message.fields {
                println!("field {}: {}", field_number, field_value)
            }
        }
        Err(e) => println!("Error parsing ISO8583 message {}", e),
    }
}

#[allow(unused_assignments)]
fn parse_message(input: &[u8]) -> Result<Message, String> {
    let (input, mti) = parse_mti(input)?;
    let (input, bitmap) = parse_bitmap(input)?;

    let mut fields = HashMap::new();
    let mut remaining = input;

    // Treat dummy field number as 1
    if let Ok((new_remaining, field_data)) = parse_variable_field(remaining) {
        fields.insert(1, field_data);
        remaining = new_remaining;
    }

    Ok(Message {
        mti,
        bitmap,
        fields,
    })
}

fn parse_mti(input: &[u8]) -> Result<(&[u8], String), String> {
    if input.len() < 4 {
        return Err("Input too short to contain MTI".to_string());
    }

    let mti_bytes = &input[..4];

    match std::str::from_utf8(mti_bytes) {
        Ok(mti) => Ok((&input[4..], mti.to_string())),
        Err(_) => Err("Invalid MTI format".to_string()),
    }
}

fn hex_to_bytes(hex: &[u8]) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string has an odd number of characters".to_string());
    }

    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i + 2];
        let byte = match std::str::from_utf8(byte_str) {
            Ok(s) => u8::from_str_radix(s, 16).map_err(|_| "Invalid hex character".to_string())?,
            Err(_) => return Err("Invalid UTF-8 sequence".to_string()),
        };
        bytes.push(byte);
    }

    Ok(bytes)
}

fn parse_bitmap(input: &[u8]) -> Result<(&[u8], Vec<u8>), String> {
    if input.len() < 16 {
        return Err("Input too short to contain bitmap".to_string());
    }

    let hex_bitmap = &input[..16];
    let bitmap = hex_to_bytes(hex_bitmap)?;

    Ok((&input[16..], bitmap))
}

// TODO: Assuming that the field length is specified as 2 digits
fn parse_variable_field(input: &[u8]) -> Result<(&[u8], String), String> {
    if input.len() < 2 {
        return Err("Input too short to contain field length".to_string());
    }

    let length_str = match std::str::from_utf8(&input[..2]) {
        Ok(s) => s,
        Err(_) => return Err("Invalid length format".to_string()),
    };

    let length: usize = match length_str.parse() {
        Ok(l) => l,
        Err(_) => return Err("Invalid length value".to_string()),
    };

    if input.len() < 2 + length {
        return Err("Input too short to contain field data".to_string());
    }

    let field_data = &input[2..2 + length];
    match std::str::from_utf8(field_data) {
        Ok(data) => Ok((&input[2 + length..], data.to_string())),
        Err(_) => Err("Invalid field data format".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mti_valid() {
        let input = b"1234rest_of_message";
        let result = parse_mti(input);
        assert_eq!(result, Ok((&b"rest_of_message"[..], "1234".to_string())))
    }

    #[test]
    fn test_parse_mti_invalid_format() {
        let input = b"\xFF\xFF\xFF\xFFrest_of_the_message";
        let result = parse_mti(input);
        assert_eq!(result, Err("Invalid MTI format".to_string()));
    }

    #[test]
    fn test_parse_mti_empty_input() {
        let input = b"";
        let result = parse_mti(input);
        assert_eq!(result, Err("Input too short to contain MTI".to_string()))
    }

    #[test]
    fn test_parse_mti_exact_length() {
        let input = b"1234";
        let result = parse_mti(input);
        assert_eq!(result, Ok((&b""[..], "1234".to_string())))
    }

    #[test]
    fn test_parse_bitmap_valid() {
        let input = b"4000000000000000rest_of_message";
        let result = parse_bitmap(input);
        assert_eq!(
            result,
            Ok((&b"rest_of_message"[..], vec![64, 0, 0, 0, 0, 0, 0, 0]))
        )
    }

    #[test]
    fn test_parse_bitmap_too_short() {
        let input = b"1234567";
        let result = parse_bitmap(input);
        assert_eq!(result, Err("Input too short to contain bitmap".to_string()));
    }

    #[test]
    fn test_parse_bitmap_exact_length() {
        let input = b"4000000000000000";
        let result = parse_bitmap(input);
        assert_eq!(result, Ok((&b""[..], vec![64, 0, 0, 0, 0, 0, 0, 0])))
    }

    #[test]
    fn test_parse_bitmap_with_non_ascii() {
        let input = b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFrest_of_message";
        let result = parse_bitmap(input);
        assert_eq!(result, Err("Invalid UTF-8 sequence".to_string()))
    }

    #[test]
    fn test_parse_variable_field_valid() {
        let input = b"041234rest_of_message";
        let result = parse_variable_field(input);
        assert_eq!(result, Ok((&b"rest_of_message"[..], "1234".to_string())))
    }

    #[test]
    fn test_parse_variable_field_invalid_length_format() {
        let input = b"\xFF\xFF1234rest_of_message";
        let result = parse_variable_field(input);
        assert_eq!(result, Err("Invalid length format".to_string()));
    }

    #[test]
    fn test_parse_variable_field_invalid_length_value() {
        let input = b"XX1234rest_of_message";
        let result = parse_variable_field(input);
        assert_eq!(result, Err("Invalid length value".to_string()));
    }

    #[test]
    fn test_parse_variable_field_too_short_for_length() {
        let input = b"04123";
        let result = parse_variable_field(input);
        assert_eq!(
            result,
            Err("Input too short to contain field data".to_string())
        );
    }

    #[test]
    fn test_parse_variable_field_invalid_data_format() {
        let input = b"04\xFF\xFF\xFF\xFFrest_of_message";
        let result = parse_variable_field(input);
        assert_eq!(result, Err("Invalid field data format".to_string()));
    }
}
