mod hash;
mod structs;

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
