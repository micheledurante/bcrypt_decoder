use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
/// $1$: MD5
/// $2$: bcrypt
/// $2a$: bcrypt
/// $2x$: bcrypt
/// $2y$: bcrypt
/// $2b$: bcrypt
/// $5$: SHA-256
/// $6$: SHA-512
/// https://stackoverflow.com/questions/5393803/can-someone-explain-how-bcrypt-verifies-a-hash
pub enum AlgoType {
    Bcrypt2,
    Bcrypt2x,
    Bcrypt2y,
    Bcrypt2a,
    Bcrypt2b,
}

impl AlgoType {
    /// Return the number for algo variants
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
    /// https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algo_type_value() {
        let cases = vec![
            ("", 0),
            ("aaa", 0),
            ("2x", 1),
            ("2y", 2),
            ("2a", 3),
            ("2b", 4),
        ];

        for (name, expect) in cases {
            assert_eq!(AlgoType::value(name), expect)
        }
    }

    #[test]
    fn test_hash_parts_new() {
        let cases = vec![
            (
                HashParts::new(1,2, "asd".into(), "qwe".into()),
                1,
                2,
                String::from("asd"),
                String::from("qwe")
            )
        ];

        for (obj, algo, cost, salt, hash) in cases {
            assert_eq!(obj.algo, algo);
            assert_eq!(obj.cost, cost);
            assert_eq!(obj.salt, salt);
            assert_eq!(obj.hash, hash);
        }
    }

    #[test]
    fn test_hash_parts_eq() {
        let cases = vec![
            (
                HashParts::new(1,2, "asd".into(), "qwe".into()),
                HashParts::new(1,2, "asd".into(), "qwe".into()),
                true
            ),
            (
                HashParts::new(2,2, "asd".into(), "qwe".into()),
                HashParts::new(1,2, "asd".into(), "qwe".into()),
                false
            )
        ];

        for (first, second, expect) in cases {
            assert_eq!(first == second, expect)
        }
    }

    #[test]
    fn test_string_utils_substring() {
        let cases = vec![
            ("", 0, 0, ""),
            ("asd", 0, 3, "asd"),
            ("asd", 1, 3, "sd"),
            ("", 1, 3, ""),
            ("asd", 4, 3, ""),
            ("asd", 3, 4, ""),
            ("asd", 2, 4, "d"),
            ("asd", 0, 4, "asd"),
        ];

        for (str, start, len, expect) in cases {
            assert_eq!(String::from(str).substring(start, len), expect)
        }
    }
}
