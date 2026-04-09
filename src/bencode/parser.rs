use crate::bencode::types::Bencode;
use std::collections::HashMap;

fn parse_data(input: &[u8], index: usize) -> Result<(Bencode, usize), String> {
    if input.is_empty() {
        return Err("Input is empty".to_string());
    }

    match input[index] {
        b'i' => parse_integer(input, index + 1),
        b'0'..=b'9' => parse_string(input, index),
        b'l' => parse_list(input, index),
        b'd' => parse_dictionary(input, index),
        _ => Err("Invalid Bencode format".to_string()),
    }
}

pub fn parse_integer(input: &[u8], index: usize) -> Result<(Bencode, usize), String> { 
    let mut end_index = index;
    while end_index < input.len() && input[end_index] != b'e' { 
        end_index += 1;
        if end_index >= input.len() {
            return Err("Unexpected end of input while parsing integer".to_string());
        }
    }
    let number_str = std::str::from_utf8(&input[index..end_index])
        .map_err(|_| "Invalid UTF-8 sequence in integer".to_string())?;
    let number = number_str
        .parse::<i64>()
        .map_err(|_| "Invalid integer format".to_string())?;
    Ok((Bencode::Integer(number), end_index + 1))
}

pub fn parse_string(input: &[u8], index: usize) -> Result<(Bencode, usize), String> { 
    let mut length_index = index;
    while length_index < input.len() && input[length_index] != b':' { 
        length_index += 1;
        if length_index >= input.len() {
            return Err("Unexpected end of input while parsing string length".to_string());
        }
    }
    let length_str = std::str::from_utf8(&input[index..length_index])
        .map_err(|_| "Invalid UTF-8 sequence in string length".to_string())?;
    let length = length_str
        .parse::<usize>()
        .map_err(|_| "Invalid String length".to_string())?;
    let start_index = length_index + 1;
    let end_index = start_index + length;
    if end_index > input.len() {
        return Err("Unexpected end of input while parsing string data".to_string());
    }
    let string_result = input[start_index..end_index].to_vec();
    Ok((Bencode::String(string_result), end_index))
}

pub fn parse_list(input: &[u8], index: usize) -> Result<(Bencode, usize), String> { 
   let mut return_list = Vec::new();
   let mut current_index = index + 1;
   while current_index < input.len() && input[current_index] != b'e' { 
        let (bencode, next_index) = parse_data(input, current_index)?;
        return_list.push(bencode);
        current_index = next_index;
    }
    if current_index >= input.len() {
        return Err("Unexpected end of input while parsing list".to_string());
    }
    Ok((Bencode::List(return_list), current_index + 1))
   }

   pub fn parse_dictionary(input: &[u8], index: usize) -> Result<(Bencode, usize), String> { 
    let mut return_dictionary = HashMap::new();
    let mut current_index = index + 1;
    while current_index < input.len() && input[current_index] != b'e' {
        let (key_bencode, next_index) = parse_string(input, current_index)?;
        let key = match key_bencode {
            Bencode::String(s) => s,
            _ => return Err("Expected string as dictionary key".to_string()),
        };
        current_index = next_index;
        let (value_bencode, next_index) = parse_data(input, current_index)?;
        return_dictionary.insert(key, value_bencode);
        current_index = next_index;
    }
    if current_index >= input.len() {
        return Err("Unexpected end of input while parsing dictionary".to_string());
    }
    Ok((Bencode::Dictionary(return_dictionary), current_index + 1))
   }