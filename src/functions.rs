/// This file contains all the functions used in the program.
/// This file is used to keep the main file clean.

/// This trait is used to convert a set of characters into a vector of Sets.
pub trait ConvertToSets {
    fn convert_to_sets(&self) -> Vec<Set>;
}

impl ConvertToSets for String {
    /// This function converts the sets of characters into a vector of Sets.
    fn convert_to_sets(&self) -> Vec<Set> {
        let mut sets = Vec::new();
        for c in self.chars() {
            match c {
                'l' => sets.push(Set::Lowercase),
                'u' => sets.push(Set::Uppercase),
                'd' => sets.push(Set::Digits),
                's' => sets.push(Set::Symbols),
                _ => (),
            }
        }
        sets
    }
}

/// A set of characters.
/// This has 4 sets of characters, lowercase, uppercase, digits, and symbols.
#[derive(Debug)]
pub enum Set {
    Lowercase,
    Uppercase,
    Digits,
    Symbols,
}

impl Set {
    /// This function returns the characters in the set.
    pub fn get_chars(&self) -> &'static str {
        match self {
            Set::Lowercase => "abcdefghijklmnopqrstuvwxyz",
            Set::Uppercase => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            Set::Digits => "0123456789",
            Set::Symbols => "!@#$%^&*()_+-=[]{};':\",./<>?`~\\",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Check if two sets are equal to eachother
    fn are_sets_equal(set1: &Set, set2: &Set) -> bool {
        matches!(
            (set1, set2),
            (Set::Lowercase, Set::Lowercase) |
            (Set::Uppercase, Set::Uppercase) |
            (Set::Digits, Set::Digits) |
            (Set::Symbols, Set::Symbols))
    }

    #[test]
    fn test_convert_to_sets() {
        let sets = "luds".to_string().convert_to_sets();
        assert_eq!(sets.len(), 4);
        assert!(are_sets_equal(&sets[0], &Set::Lowercase));
        assert!(are_sets_equal(&sets[1], &Set::Uppercase));
        assert!(are_sets_equal(&sets[2], &Set::Digits));
        assert!(are_sets_equal(&sets[3], &Set::Symbols));
    }

    #[test]
    fn test_get_sets() {
        assert_eq!(Set::Lowercase.get_chars(), "abcdefghijklmnopqrstuvwxyz");
        assert_eq!(Set::Uppercase.get_chars(), "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        assert_eq!(Set::Digits.get_chars(), "0123456789");
        assert_eq!(Set::Symbols.get_chars(), "!@#$%^&*()_+-=[]{};':\",./<>?`~\\");
    }
}
