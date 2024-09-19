fn main() {
    println!("Hello, world!");
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

fn parse_bitmap(input: &[u8]) -> Result<(&[u8], Vec<u8>), String> {
    if input.len() < 8 {
        return Err("Input too short to contain bitmap".to_string());
    }

    let bitmap = input[..8].to_vec();
    Ok((&input[8..], bitmap))
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
        let input = b"12345678rest_of_message";
        let result = parse_bitmap(input);
        assert_eq!(result, Ok((&b"rest_of_message"[..], b"12345678".to_vec())))
    }

    #[test]
    fn test_parse_bitmap_too_short() {
        let input = b"1234567";
        let result = parse_bitmap(input);
        assert_eq!(result, Err("Input too short to contain bitmap".to_string()));
    }

    #[test]
    fn test_parse_bitmap_exact_length() {
        let input = b"12345678";
        let result = parse_bitmap(input);
        assert_eq!(result, Ok((&b""[..], b"12345678".to_vec())))
    }

    #[test]
    fn test_parse_bitmap_with_non_ascii() {
        let input = b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFrest_of_message";
        let result = parse_bitmap(input);
        assert_eq!(
            result,
            Ok((
                &b"rest_of_message"[..],
                b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF".to_vec()
            ))
        )
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
