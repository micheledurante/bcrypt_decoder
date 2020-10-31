use crate::structs::{HashParts, StringUtils, AlgoType};

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
fn create_result(parts: Vec<&str>) -> HashParts {
    HashParts::new(
        AlgoType::value(parts[1]) as u32,
        parts[2].parse::<u32>().unwrap(),
        String::from(parts[3]).substring(0, 22),
        String::from(parts[3]).substring(22, 31)
    )
}

/// Create an return the HashParts struct for the given hash.
fn split_hash_into_parts(hash: &str) -> Option<HashParts> {
    if !validate_hash(hash) {
        return None;
    }

    Some(create_result(split_dollar_signs(hash)))
}

pub fn get_algo(hash: &str) -> u32 {
   split_hash_into_parts(hash).unwrap().algo()
}

pub fn get_cost(hash: &str) -> u32 {
    split_hash_into_parts(hash).unwrap().cost()
}

pub fn get_salt(hash: &str) -> String {
    split_hash_into_parts(hash).unwrap().salt()
}

pub fn get_hash(hash: &str) -> String {
    split_hash_into_parts(hash).unwrap().hash()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_hash_into_parts() {
        let cases = vec![
            (
                "$3$10$Ro0CUfOqk6cXEKf3dyaM7OhSCvnwM9s4wIX9JeLapehKK5YdLxKcm",
                Some(HashParts::new(3, 10, "Ro0CUfOqk6cXEKf3dyaM7O".into(), "hSCvnwM9s4wIX9JeLapehKK5YdLxKcm".into()))
            )
        ];

        for (hash, expect) in cases {
            assert_eq!(
                split_hash_into_parts(hash),
                expect
            )
        }
    }

    #[test]
    fn test_create_result() {
        let cases = vec![
            (
                vec!["", "1", "2", "Ro0CUfOqk6cXEKf3dyaM7OhSCvnwM9s4wIX9JeLapehKK5YdLxKcm"],
                HashParts::new(1, 2, "Ro0CUfOqk6cXEKf3dyaM7O".into(), "hSCvnwM9s4wIX9JeLapehKK5YdLxKcm".into())
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
