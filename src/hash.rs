use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub enum AlgoType {
    BCrypt,
    BCrypt2X,
    BCrypt2Y,
    BCrypt2B,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct HashParts {
    algo: AlgoType,
    cost: u32,
}

#[wasm_bindgen]
impl HashParts {
    #[wasm_bindgen(constructor)]
    pub fn new(algo: AlgoType, cost: u32) -> Self {
        HashParts { algo, cost }
    }

    #[wasm_bindgen(getter)]
    pub fn cost(&self) -> u32 {
        self.cost
    }

    #[wasm_bindgen(getter)]
    pub fn algo(self) -> AlgoType {
        self.algo
    }
}

impl PartialEq for HashParts {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

/// Bcrypt hashes are either 59 or 60 characters long, depending on the bcrypt variant used:
/// $1$: MD5
/// $2$: bcrypt
/// $2a$: bcrypt
/// $2x$: bcrypt
/// $2y$: bcrypt
/// $2b$: bcrypt
/// $5$: SHA-256
/// $6$: SHA-512
fn validate_hash(hash: &str) -> bool {
    let mut result = false;

    if hash.len() == 59 || hash.len() == 60 {
        result = true;
    }

    result
}

/// Split the hash and return its parts ina readable form.
pub fn get_hash_parts(hash: &str) -> HashParts {
    let parts = HashParts::new(AlgoType::BCrypt, 10);
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_hash_too_short() {
        let cases = vec![
            ("asd", false),
            ("", false)
        ];

        for (hash, expect) in cases {
            assert_eq!(
                validate_hash(hash),
                expect
            )
        }
    }

    #[test]
    fn test_validate_hash_too_long() {
        let string = "asdnhqanfwoefnmwepfmweddweifdmewofnowefwefdweifdmewofnowefwef";
        assert_eq!(false, validate_hash(string));
    }

    #[test]
    fn test_validate_hash_ok() {
        let cases = vec![
            ("asdnhqanfwoefnmwepfmweddweifdmewofnowefwefdweifdmewofnowefwe", true),
            ("asdnhqanfwoefnmwepfmweddweifdmewofnowefwefdweifdmewofnowefe", true)
        ];

        for (hash, expect) in cases {
            assert_eq!(
                validate_hash(hash),
                expect
            )
        }
    }
}
