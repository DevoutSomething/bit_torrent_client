#[cfg(test)]
mod tests {
    use crate::bencode::parser::{parse_integer, parse_string, parse_list, parse_dictionary};
    use crate::bencode::encode::encode;
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

    #[test]
fn test_parse_dictionary_empty() {
    let input = b"de";
    let result = parse_dictionary(input, 0);

    assert!(result.is_ok());
    let (bencode, index) = result.unwrap();

    assert_eq!(index, 2);

    match bencode {
        Bencode::Dictionary(map) => assert!(map.is_empty()),
        _ => panic!("Expected Dictionary"),
    }
}

#[test]
fn test_parse_dictionary_simple() {
    let input = b"d3:cow3:mooe";
    let result = parse_dictionary(input, 0);

    assert!(result.is_ok());
    let (bencode, _) = result.unwrap();

    match bencode {
        Bencode::Dictionary(map) => {
            let value = map.get(&b"cow".to_vec()).unwrap();

            match value {
                Bencode::String(s) => assert_eq!(s, b"moo"),
                _ => panic!("Expected String"),
            }
        }
        _ => panic!("Expected Dictionary"),
    }
}

#[test]
fn test_parse_dictionary_multiple_pairs() {
    let input = b"d3:bar4:spam3:fooi42ee";
    let result = parse_dictionary(input, 0);

    assert!(result.is_ok());
    let (bencode, _) = result.unwrap();

    match bencode {
        Bencode::Dictionary(map) => {
            assert_eq!(map.len(), 2);

            match map.get(&b"foo".to_vec()).unwrap() {
                Bencode::Integer(n) => assert_eq!(*n, 42),
                _ => panic!("Expected Integer"),
            }
        }
        _ => panic!("Expected Dictionary"),
    }
}

#[test]
fn test_parse_list_empty() {
    let input = b"le";
    let result = parse_list(input, 0);

    assert!(result.is_ok());
    let (bencode, index) = result.unwrap();

    assert_eq!(index, 2);

    match bencode {
        Bencode::List(v) => assert!(v.is_empty()),
        _ => panic!("Expected List"),
    }
}

#[test]
fn test_parse_list_integers() {
    let input = b"li1ei2ei3ee";
    let result = parse_list(input, 0);

    assert!(result.is_ok());
    let (bencode, index) = result.unwrap();

    assert_eq!(index, input.len());

    match bencode {
        Bencode::List(v) => {
            assert_eq!(v.len(), 3);

            match &v[0] {
                Bencode::Integer(n) => assert_eq!(*n, 1),
                _ => panic!("Expected Integer"),
            }
        }
        _ => panic!("Expected List"),
    }
}

#[test]
fn test_parse_list_strings() {
    let input = b"l3:foo3:bare";
    let result = parse_list(input, 0);

    assert!(result.is_ok());
    let (bencode, _) = result.unwrap();

    match bencode {
        Bencode::List(v) => {
            assert_eq!(v.len(), 2);

            match &v[0] {
                Bencode::String(s) => assert_eq!(s, b"foo"),
                _ => panic!("Expected String"),
            }
        }
        _ => panic!("Expected List"),
    }
}

#[test]
fn test_encode_dictionary() {
    let mut dict = std::collections::HashMap::new();
    dict.insert(b"cow".to_vec(), Bencode::String(b"moo".to_vec()));
    let bencode = Bencode::Dictionary(dict);
    let encoded = encode(&bencode);
    assert_eq!(encoded, b"d3:cow3:mooe");
}

#[test]
fn test_encode_integer() {
    let bencode = Bencode::Integer(42);
    let encoded = encode(&bencode);
    assert_eq!(encoded, b"i42e");
}

#[test]
fn test_encode_string() {
    let bencode = Bencode::String(b"hello".to_vec());
    let encoded = encode(&bencode);
    assert_eq!(encoded, b"5:hello");
}

#[test]
fn test_encode_list() {
    let list = vec![Bencode::Integer(1), Bencode::Integer(2), Bencode::Integer(3)];
    let bencode = Bencode::List(list);
    let encoded = encode(&bencode);
    assert_eq!(encoded, b"li1ei2ei3ee");
}

#[test]
fn test_encode_complex_dictionary() {
    let mut dict = std::collections::HashMap::new();
    dict.insert(b"bar".to_vec(), Bencode::String(b"spam".to_vec()));
    dict.insert(b"foo".to_vec(), Bencode::Integer(42));
    let bencode = Bencode::Dictionary(dict);
    let encoded = encode(&bencode);
    // Dictionaries are sorted by key
    assert_eq!(encoded, b"d3:bar4:spam3:fooi42ee");
}
}           

