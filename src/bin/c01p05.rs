/*
 * PROBLEM:
 * One Away: There are three types of edits that can be performed on strings: insert a character,
 * remove a character, or replace a character. Given two strings, write a function to check if they
 * are one edit (or zero edits) away.
 * EXAMPLE:
 * pale, ple -> true
 * pales, pale -> true
 * pale, bale -> true
 * pale, bake -> false
 */

fn is_one_away(s1: &str, s2: &str) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_one_way() {
        assert!(is_one_away("Hello", "Hllo"));
    }
}

fn main() {
    is_one_away("Hello", "Hllo");
}
