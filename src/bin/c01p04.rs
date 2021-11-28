/*
 * PROBLEM:
 * Palindrome Permutation: Given a string, write a function to check if it is a permutation of a
 * palindrome. A palindrome is a word or phrase that is the same forwards and backwards. A
 * permutation is a rearrangement of letters. The palindrome does not need to be limited to just
 * dictionary words.
 * EXAMPLE:
 * Input: Tact Coa
 * Output: True (permutations: "taco cat", "atco cta", etc.)
 */

fn is_palindrome_permutation(s: &str) -> bool {
    true
}

fn main() {
    is_palindrome_permutation("Tact Coa");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_permutation() {
        assert!(is_palindrome_permutation("Tact Coa"));
    }
}
