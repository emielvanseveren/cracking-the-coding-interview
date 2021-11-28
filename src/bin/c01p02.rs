/*
 * PROBLEM:
 * Check Permutation: Given two strings, write a method to decide if one is a permutation of the
 * other.
*/

fn is_permutation(s1: &str, s2: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        assert!(is_permutation("abc", "cab"));
        assert!(!is_permutation("abc", "abd"));
    }
}

fn main() {
    is_permutation("abc", "cab");
}
