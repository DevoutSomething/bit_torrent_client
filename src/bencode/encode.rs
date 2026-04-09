use crate::bencode::types::Bencode;

pub fn encode(b: &Bencode) -> Vec<u8> { 
    match b {
        Bencode::Integer(i) => {
            let mut result = Vec::new();
            result.push(b'i');
            result.extend_from_slice(i.to_string().as_bytes());
            result.push(b'e');
            result
        }
        Bencode::String(s) => {
            let mut result = Vec::new();
            result.extend_from_slice(s.len().to_string().as_bytes());
            result.push(b':');
            result.extend_from_slice(s);
            result
        }
        Bencode::List(l) => {
            let mut result = Vec::new();
            result.push(b'l');
            for item in l {
                result.extend_from_slice(&encode(item));
            }
            result.push(b'e');
            result
        }
        Bencode::Dictionary(d) => {
            let mut result = Vec::new();
            result.push(b'd');
            let mut sorted_keys: Vec<_> = d.keys().collect();
            sorted_keys.sort();
            for key in sorted_keys {
                let value = &d[key];
                result.extend_from_slice(&encode(&Bencode::String(key.clone())));
                result.extend_from_slice(&encode(value));
            }
            result.push(b'e');
            result
        }
    }
}