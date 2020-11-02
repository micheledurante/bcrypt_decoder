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
    UNKNOWN,
    Bcrypt2 ,
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
            "2" => Self::Bcrypt2 as u8,
            _ => Self::UNKNOWN as u8,
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
        assert_eq!(AlgoType::value(""), 0);
        assert_eq!(AlgoType::value("6"), 0);
        assert_eq!(AlgoType::value("aaa"), 0);
        assert_eq!(AlgoType::value("2"), 1);
        assert_eq!(AlgoType::value("2x"), 2);
        assert_eq!(AlgoType::value("2y"), 3);
        assert_eq!(AlgoType::value("2a"), 4);
        assert_eq!(AlgoType::value("2b"), 5);
    }

    #[test]
    fn test_hash_parts_new() {
        let obj = HashParts::new(1,2, "asd".into(), "qwe".into());

        assert_eq!(obj.algo, 1);
        assert_eq!(obj.cost, 2);
        assert_eq!(obj.salt, String::from("asd"));
        assert_eq!(obj.hash, String::from("qwe"));
    }

    #[test]
    fn test_hash_parts_eq() {
        assert_eq!(
            HashParts::new(1,2, "asd".into(), "qwe".into()) == HashParts::new(1,2, "asd".into(), "qwe".into()),
            true
        );

        assert_eq!(
            HashParts::new(2,2, "asd".into(), "qwe".into()) == HashParts::new(1,2, "asd".into(), "qwe".into()),
            false
        );
    }

    #[test]
    fn test_string_utils_substring() {
        assert_eq!(String::from("").substring(0, 0), "");
        assert_eq!(String::from("asd").substring(0, 3), "asd");
        assert_eq!(String::from("asd").substring(1, 3), "sd");
        assert_eq!(String::from("").substring(1, 3), "");
        assert_eq!(String::from("asd").substring(4, 3), "");
        assert_eq!(String::from("asd").substring(3, 4), "");
        assert_eq!(String::from("asd").substring(2, 4), "d");
        assert_eq!(String::from("asd").substring(0, 4), "asd");
    }
}
