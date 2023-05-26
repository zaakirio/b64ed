use base64::{Engine as _, engine::general_purpose};
use std::str;

pub fn encode(input: &String) -> String {
    return general_purpose::STANDARD.encode(input);

}

pub fn decode(input: &String) -> String {
    let bytes = general_purpose::STANDARD.decode(input).unwrap();
    return str::from_utf8(&bytes).unwrap().to_string();

}