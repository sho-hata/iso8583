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
}
