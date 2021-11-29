/*
 * PROBLEM:
 * Is Unique: Implement an algorithm to determine if a string has all unique characters.
 * What if you cannot use additional data structures?
*/

// It would be even better if we used a BitSet here.
// Unfortunately BitSet is not part of the std library at this moment in time.
fn all_chars_unique_part_a(s: &str) -> bool {
    use std::collections::HashSet;
    let mut chars: HashSet<char> = HashSet::with_capacity(s.len());

    for c in s.chars() {
        if chars.contains(&c) {
            return false;
        }
        chars.insert(c);
    }
    true
}

// No additional data structures
// Order String (O(n*log(n))
// Create a bitset from an integer.
fn all_chars_unique_part_b(s: &str) -> bool {
    let mut bit_set: i64 = 0;
    let a_int_char: i16 = 'a' as i16;

    for c in s.chars() {
        let mut int_char: i16 = c as i16;

        // if char is a it becomes (1), other characters are moved 'a' positions.
        int_char -= a_int_char;

        // check if bit from certain character is already set.
        if (1 << int_char) & bit_set != 0 {
            return false;
        }
        bit_set |= 1 << int_char;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert!(all_chars_unique_part_a("abcdefg"));
        assert!(!all_chars_unique_part_a("abcdefgabc"));
    }

    #[test]
    fn test_part_b() {
        assert!(all_chars_unique_part_b("abcdefg"));
        assert!(!all_chars_unique_part_b("abcdefgabc"));
    }
}

fn main() {
    all_chars_unique_part_a(&String::from("Hello world"));
    all_chars_unique_part_b(&String::from("Hello world"));
}
