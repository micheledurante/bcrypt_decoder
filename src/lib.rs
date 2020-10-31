mod hash;

use wasm_bindgen::prelude::*;
use hash::HashParts;

#[wasm_bindgen]
pub fn get_hash_parts(hash: &str) -> HashParts {
    hash::get_hash_parts(hash)
}
