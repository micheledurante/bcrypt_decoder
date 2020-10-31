mod hash;

use wasm_bindgen::prelude::*;
use hash::Parts;

#[wasm_bindgen]
pub fn get_hash_parts(hash: &str) -> Parts {
    hash::get_hash_parts(hash)
}
