mod hash;
mod structs;
mod base64;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_algo(hash: &str) -> u32 {
    hash::get_algo(hash)
}

#[wasm_bindgen]
pub fn get_cost(hash: &str) -> u32 {
    hash::get_cost(hash)
}

#[wasm_bindgen]
pub fn get_salt(hash: &str) -> String {
    hash::get_salt(hash)
}

#[wasm_bindgen]
pub fn get_hash(hash: &str) -> String {
    hash::get_hash(hash)
}

#[wasm_bindgen]
pub fn get_salt_bytes(hash: &str) -> Vec<u8> {
    hash::get_salt_bytes(hash)
}

#[wasm_bindgen]
pub fn get_hash_bytes(hash: &str) -> Vec<u8> {
    hash::get_hash_bytes(hash)
}
