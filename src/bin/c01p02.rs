/*
 * PROBLEM:
 * Check Permutation: Given two strings, write a method to decide if one is a permutation of the
 * other.
*/

// We could order the letters of the string and compare them O(n*lg(n)).
// If the string are equal they are permutations.
fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s1 = s1.chars().collect::<Vec<_>>();
    s1.sort_unstable();

    let mut s2 = s2.chars().collect::<Vec<_>>();
    s2.sort_unstable();

    if s1 == s2 {
        return true;
    }
    false
}

// Another implementation could be creating 2 sets, and intersect them.
// If the intersection is empty, we can consider a permutation.
// No clue what the O is for the intersect.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        assert!(is_permutation("abc", "cab"));
        assert!(!is_permutation("abc", "abd"));
        assert!(!is_permutation("abcd", "abc"));
    }
}

fn main() {
    is_permutation("abc", "cab");
}
