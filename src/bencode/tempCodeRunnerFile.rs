pub fn parse_integer(input: &[u8], index: usize) -> Result<(Bencode, usize), String> { 
    let mut end_index = index  + 1;
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