/// Decode a string from bcrypt base64 encoding
/// https://stackoverflow.com/questions/41034635/idiomatic-transformations-for-string-str-vecu8-and-u8
pub fn decode_bcrypt(input: String) -> Vec<u8> {
    base64::decode_config(input, base64::BCRYPT).unwrap()
}
