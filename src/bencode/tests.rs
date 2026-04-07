#[cfg(test)]
mod tests {
    use crate::bencode::parser::{parse_integer, parse_string};
    use crate::bencode::types::Bencode;

    #[test]
    fn test_parse_integer_negative() {
        let input = b"-123e";
        let result = parse_integer(input, 0);
        assert!(result.is_ok());
        let (bencode, index) = result.unwrap();
        assert_eq!(index, 5);
        match bencode {
            Bencode::Integer(n) => assert_eq!(n, -123),
            _ => panic!("Expected Integer"),
        }
    }

    #[test]
    fn test_parse_integer_zero() {
        let input = b"0e";
        let result = parse_integer(input, 0);
        assert!(result.is_ok());
        let (bencode, index) = result.unwrap();
        assert_eq!(index, 2);
        match bencode {
            Bencode::Integer(n) => assert_eq!(n, 0),
            _ => panic!("Expected Integer"),
        }
    }

    #[test]
    fn test_parse_integer_large() {
        let input = b"123456789e";
        let result = parse_integer(input, 0);
        assert!(result.is_ok());
        let (bencode, index) = result.unwrap();
        assert_eq!(index, 10);
        match bencode {
            Bencode::Integer(n) => assert_eq!(n, 123456789),
            _ => panic!("Expected Integer"),
        }
    }

    #[test]
    fn test_parse_integer_invalid() {
        let input = b"abc";
        let result = parse_integer(input, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_string_simple() {
        let input = b"5:hello";
        let result = parse_string(input, 0);
        assert!(result.is_ok());
        let (bencode, index) = result.unwrap();
        assert_eq!(index, 7);
        match bencode {
            Bencode::String(s) => assert_eq!(s, b"hello"),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_parse_string_empty() {
        let input = b"0:";
        let result = parse_string(input, 0);
        assert!(result.is_ok());
        let (bencode, index) = result.unwrap();
        assert_eq!(index, 2);
        match bencode {
            Bencode::String(s) => assert_eq!(s, b""),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_parse_string_with_spaces() {
        let input = b"11:hello world";
        let result = parse_string(input, 0);
        assert!(result.is_ok());
        let (bencode, index) = result.unwrap();
        assert_eq!(index, 14);
        match bencode {
            Bencode::String(s) => assert_eq!(s, b"hello world"),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_parse_string_invalid_length() {
        let input = b"abc:hello";
        let result = parse_string(input, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_string_too_short() {
        let input = b"5:hi";
        let result = parse_string(input, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_integer_positive() {
        let input = b"42e";
        let result = parse_integer(input, 0);
        assert!(result.is_ok());
        let (bencode, index) = result.unwrap();
        assert_eq!(index, 3);
        match bencode {
            Bencode::Integer(n) => assert_eq!(n, 42),
            _ => panic!("Expected Integer"),
        }
    }

    #[test]
    fn test_parse_integer_leading_zero() {
        let input = b"042e"; // leading zero is allowed in bencode
        let result = parse_integer(input, 0);
        assert!(result.is_ok());
        let (bencode, index) = result.unwrap();
        assert_eq!(index, 4);
        match bencode {
            Bencode::Integer(n) => assert_eq!(n, 42),
            _ => panic!("Expected Integer"),
        }
    }
}           