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

// if the length is an even number all used characters should (atleast) exist twice
// if the length is an odd number all used characters should (atleast) exist twice except for one
// that should only exist once.
// lepel = spoon
//
use std::collections::HashMap;

fn count_characters(s: &str) -> HashMap<char, i32> {
    let mut chars: HashMap<char, i32> = HashMap::with_capacity(s.len() / 2 + 1);
    for c in s.chars() {
        let x = chars.entry(c).or_insert(0);
        *x += 1;
    }
    chars
}
fn is_palindrome_permutation(s: &str) -> bool {
    // Lets start by normalizing the string
    let normalized = s
        .to_lowercase()
        .split_whitespace()
        .fold(String::new(), |acc, s| acc + s);

    let char_counts = count_characters(s);
    let mut has_odd = false;
    let is_even = normalized.len() % 2 == 0;

    // lets now loop the map

    for val in char_counts.values() {
        if val % 2 != 0 {
            if is_even || has_odd {
                return false;
            } else {
                has_odd = true;
            }
        }
    }
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
