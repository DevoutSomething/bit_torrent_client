use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Bencode {
    Integer(i64),
    String(Vec<u8>),
    List(Vec<Bencode>),
    Dictionary(HashMap<String, Bencode>),
}
