/*
 * PROBLEM:
 * write a function that takes a string and substring and returns the index of the first occurence
 * of the substring in the string.
 *
*/

fn substring(str: &str, substr: &str) -> i8 {
    0
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
        assert_eq!(substring("hello", "el"), 3);
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
