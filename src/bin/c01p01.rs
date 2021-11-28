/*
 * PROBLEM:
 * Is Unique: Implement an algorithm to determine if a string has all unique characters.
 * What if you cannot use additional data structures?
*/

fn all_chars_unique_part_a(s: &str) -> bool {
    true
}

fn all_chars_unique_part_b(s: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {}

    #[test]
    fn test_part_b() {}
}

fn main() {
    all_chars_unique_part_a(&String::from("Hello world"));
    all_chars_unique_part_b(&String::from("Hello world"));
}
