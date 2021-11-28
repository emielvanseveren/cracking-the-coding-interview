/*
 * PROBLEM:
 * String Rotation: Assume you have a method isSubstring which checks if one word is a substring of
 * another. Given two strings, s1 and s2, write code to check if s2 is a rotation of s1 using only
 * one call to isSubstring (e.g., "waterbottle" is a rotation of "erbottlewat").
*/

fn is_rotation(s1: &str, s2: &str) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rotation() {
        assert!(is_rotation("waterbottle", "erbottlewat"));
        assert!(!is_rotation("waterbottle", "erbottlewaa"));
    }
}

fn main() {
    is_rotation("waterbottle", "erbottlewat");
}
