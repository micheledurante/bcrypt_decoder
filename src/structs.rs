use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum AlgoType {
    Bcrypt2,
    Bcrypt2x,
    Bcrypt2y,
    Bcrypt2a,
    Bcrypt2b,
}

impl AlgoType {
    pub fn value(source: &str) -> u8 {
        match source {
            "2a" => Self::Bcrypt2a as u8,
            "2x" => Self::Bcrypt2x as u8,
            "2y" => Self::Bcrypt2y as u8,
            "2b" => Self::Bcrypt2b as u8,
            _ => Self::Bcrypt2 as u8
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct HashParts {
    algo: u32,
    cost: u32,
    salt: String,
    hash: String
}

#[wasm_bindgen]
impl HashParts {
    #[wasm_bindgen(constructor)]
    pub fn new(algo: u32, cost: u32, salt: String, hash: String) -> Self {
        HashParts { algo, cost, salt, hash }
    }

    pub fn algo(&self) -> u32 {
        self.algo
    }

    pub fn cost(&self) -> u32 {
        self.cost
    }

    pub fn salt(self) -> String {
        self.salt
    }

    pub fn hash(self) -> String {
        self.hash
    }
}

impl PartialEq for HashParts {
    fn eq(&self, other: &Self) -> bool {
        self.algo == other.algo &&
            self.cost == other.cost &&
            self.salt == other.salt &&
            self.hash == other.hash
    }
}

pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}
