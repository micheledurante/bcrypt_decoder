use crate::structs::{HashParts, StringUtils, AlgoType};

/// 22 character salt + 31 character hash
fn valid_salt_and_hash(salt_and_hash: &str) -> bool {
    salt_and_hash.len() == 53
}

/// The cost must be between 4 and 31 inclusive.
fn valid_cost(cost: &str) -> bool {
    cost.parse::<u32>().unwrap() >= 4 && cost.parse::<u32>().unwrap() <= 31
}

/// Must be a valid name.
fn valid_algo(algo: &str) -> bool {
    AlgoType::value(algo) != 0
}

/// Validate the parts that were separated by the `$` sign.
fn valid_parts(parts: &Vec<&str>) -> bool {
    parts.len() == 4 && parts[0] == "" &&
        valid_algo(parts[1]) && valid_cost(parts[2]) &&
        valid_salt_and_hash(parts[3])
}

/// Can be 59 or 60 chars long, depending on the variant used.
fn valid_hash(hash: &str) -> bool {
    hash.len() == 59 || hash.len() == 60
}

/// Split the hash into its meaningful parts.
fn split_dollar_signs(hash: &str) -> Vec<&str> {
    hash.split("$").collect()
}

/// Create the struct from the hash parts.
fn create_result(parts: Vec<&str>) -> HashParts {
    HashParts::new(
        AlgoType::value(parts[1]) as u32,
        parts[2].parse::<u32>().unwrap(),
        String::from(parts[3]).substring(0, 22),
        String::from(parts[3]).substring(22, 31)
    )
}

/// Create and return the struct for the given hash.
fn split_hash_into_parts(hash: &str) -> Option<HashParts> {
    let parts = split_dollar_signs(hash);

    if !valid_hash(hash) || !valid_parts(&parts) {
        return None;
    }

    Some(create_result(parts))
}

/// Get the algo used in the given hash
pub fn get_algo(hash: &str) -> u32 {
   split_hash_into_parts(hash).unwrap().algo()
}

/// Get the cost used in the given hash
pub fn get_cost(hash: &str) -> u32 {
    split_hash_into_parts(hash).unwrap().cost()
}

/// Get the salt used in the given hash
pub fn get_salt(hash: &str) -> String {
    split_hash_into_parts(hash).unwrap().salt()
}

/// Get the hashed password in the given hash
pub fn get_hash(hash: &str) -> String {
    split_hash_into_parts(hash).unwrap().hash()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_hash_into_parts() {
        assert_eq!(
            split_hash_into_parts("$2a$10$Ro0CUfOqk6cXEKf3dyaM7OhSCvnwM9s4wIX9JeLapehKK5YdLxKcm"),
            Some(HashParts::new(4, 10, "Ro0CUfOqk6cXEKf3dyaM7O".into(), "hSCvnwM9s4wIX9JeLapehKK5YdLxKcm".into()))
        );
        assert_eq!(
            split_hash_into_parts(""),
            None
        );
    }

    #[test]
    fn test_create_result() {
        assert_eq!(
            create_result(vec!["", "1", "2", "Ro0CUfOqk6cXEKf3dyaM7OhSCvnwM9s4wIX9JeLapehKK5YdLxKcm"]),
            HashParts::new(0, 2, "Ro0CUfOqk6cXEKf3dyaM7O".into(), "hSCvnwM9s4wIX9JeLapehKK5YdLxKcm".into())
        );
    }

    #[test]
    fn test_split_dollar_signs() {
        assert_eq!(split_dollar_signs(""), vec![""]);
        assert_eq!(split_dollar_signs("a"), vec!["a"]);
        assert_eq!(split_dollar_signs("$a"), vec!["", "a"]);
        assert_eq!(split_dollar_signs("$a$b"), vec!["", "a", "b"]);
        assert_eq!(split_dollar_signs("$a$b$asd"), vec!["", "a", "b", "asd"]);
        assert_eq!(split_dollar_signs("$$asd"), vec!["", "", "asd"]);
        assert_eq!(split_dollar_signs("$$a$asd"), vec!["", "", "a", "asd"]);
    }

    #[test]
    fn test_valid_hash() {
        assert_eq!(valid_hash(""), false);
        assert_eq!(valid_hash("asdasdadsasadasdasdasdasdasdasdasdasdasdasdasdasdasdasdasasd"), true);
        assert_eq!(valid_hash("asdasdadsasadasdasdasdasdasdasdasdasdasdasdasdasdasdasdaasd"), true);
    }

    #[test]
    fn test_valid_parts() {
        assert_eq!(valid_parts(&vec![""]), false);
        assert_eq!(valid_parts(&vec!["a", "a", "b", "asd"]), false);
        assert_eq!(valid_parts(&vec!["", "", "", ""]), false);
        assert_eq!(valid_parts(&vec!["", "1", "", ""]), false);
        assert_eq!(valid_parts(&vec!["", "1", "2", ""]), false);
        assert_eq!(valid_parts(&vec!["", "1", "", "asd"]), false);
        assert_eq!(valid_parts(&vec!["", "", "2", "asd"]), false);
        assert_eq!(valid_parts(&vec!["", "", "", "asd"]), false);
        assert_eq!(valid_parts(&vec!["", "2a", "3", "Ro0CUfOqk6cXEKf3dyaM7OhSCvnwM9s4wIX9JeLapehKK5YdLxKcm"]), false);
        assert_eq!(valid_parts(&vec!["", "2a", "32", "Ro0CUfOqk6cXEKf3dyaM7OhSCvnwM9s4wIX9JeLapehKK5YdLxKcm"]), false);
        assert_eq!(valid_parts(&vec!["", "2a", "10", "Ro0CUfOqk6cXEKf3dyaM7OhSCvnwM9s4wIX9JeLapehKK5YdLxKcm"]), true);
    }
}
