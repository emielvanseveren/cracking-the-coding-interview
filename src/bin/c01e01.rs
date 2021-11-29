/*
 * PROBLEM:
 * write a function that takes a string and substring and returns the index of the first occurence
 * of the substring in the string.
 *
*/

// We can use the Rabin-Karp Substring Search Algorithm to solve this problem.
fn substring(str: &str, substr: &str) -> i8 {
    use std::collections::hash_map::DefaultHasher;
    use std::convert::TryFrom;
    use std::hash::{Hash, Hasher};

    if substr.len() > str.len() {
        return -1;
    }

    let mut hasher = DefaultHasher::new();
    substr.hash(&mut hasher);
    let substr_hash = hasher.finish();

    for (i, _) in str.chars().enumerate() {
        if i < str.len() - substr.len() + 1 {
            let mut hasher = DefaultHasher::new();
            str.chars()
                .skip(i)
                .take(substr.len())
                .collect::<String>()
                .hash(&mut hasher);

            if hasher.finish() == substr_hash {
                return i8::try_from(i).unwrap();
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_substring_same() {
        assert_eq!(substring("hello", "hello"), 0);
    }

    #[test]
    fn test_substring_beginning() {
        assert_eq!(substring("hello", "hel"), 0);
    }

    #[test]
    fn test_substring_middle() {
        assert_eq!(substring("hello", "el"), 1);
    }

    #[test]
    fn test_substring_end() {
        assert_eq!(substring("hello", "lo"), 3);
    }

    #[test]
    fn test_substring_partial_match() {
        assert_eq!(substring("ababac", "abac"), 2);
    }

    #[test]
    fn test_substring_not_found() {
        assert_eq!(substring("hello", "goodbye"), -1);
    }
}

fn main() {
    substring("hello", "hel");
}
