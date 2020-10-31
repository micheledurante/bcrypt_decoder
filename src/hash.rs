use wasm_bindgen::prelude::*;
use std::str::Split;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum AlgoType {
    Bcrypt2,
    Bcrypt2x,
    Bcrypt2y,
    Bcrypt2a,
    Bcrypt2b,
}

#[wasm_bindgen]
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

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

/// Bcrypt hashes will be either 59 or 60 characters long, depending on the bcrypt variant used:
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

/// Split the given hash into its parts
fn split_dollar_signs(hash: &str) -> Vec<&str> {
    hash.split("$").collect()
}

/// Create the resulting struct from the hash parts
fn create_result(parts: Vec<&str>) -> (u32, u32, String, String) {
    (
        parts[1].parse::<u32>().unwrap(),
        parts[2].parse::<u32>().unwrap(),
        String::from(parts[3]).substring(0, 22),
        String::from(parts[3]).substring(22, 31)
    )
}

fn split_hash_into_parts(hash: &str) -> Option<(u32, u32, String, String)> {
    if !validate_hash(hash) {
        return None;
    }

    let parts  = split_dollar_signs(hash);

    Some(create_result(parts))
}

// pub fn get_algo(hash: &str) -> Option<AlgoType> {
//     let algo: u23 = split_hash_into_parts(hash);
// }

// pub fn get_cost(hash: &str) -> u32 {
//
// }
//
// pub fn get_salt(hash: &str) -> &str {
//
// }
//
// pub fn get_hash(hash: &str) -> String {
//
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_result() {
        let cases = vec![
            (
                vec!["", "1", "2", "Ro0CUfOqk6cXEKf3dyaM7OhSCvnwM9s4wIX9JeLapehKK5YdLxKcm"],
                (1, 2, "Ro0CUfOqk6cXEKf3dyaM7O".into(), "hSCvnwM9s4wIX9JeLapehKK5YdLxKcm".into())
            )
        ];

        for (parts, expect) in cases {
            assert_eq!(
                create_result(parts),
                expect
            )
        }
    }

    #[test]
    fn test_split_dollar_signs() {
        let cases = vec![
            ("", vec![""]),
            ("a", vec!["a"]),
            ("$a", vec!["", "a"]),
            ("$a$b", vec!["", "a", "b"]),
            ("$a$b$asd", vec!["", "a", "b", "asd"]),
            ("$$asd", vec!["", "", "asd"]),
            ("$$a$asd", vec!["", "", "a", "asd"]),
        ];

        for (hash, expect) in cases {
            assert_eq!(
                split_dollar_signs(hash),
                expect
            )
        }
    }

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
