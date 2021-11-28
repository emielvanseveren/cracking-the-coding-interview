/*
 * PROBLEM:
 * URLify: Write a method to replace all spaces in a string with '%20'. You may assume that the
 * string has sufficient space at the end to hold the additional characters, and that you are given
 * the "true" length of the string.
 * EXAMPLE:
 * Input: "Mr John Smith    ", 13
 * Output: "Mr%20John%20Smith"
 */

fn urlify(s: &str) -> &str {
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlify() {
        assert_eq!(urlify("Mr John Smith    "), "Mr%20John%20Smith");
    }
}

fn main() {
    urlify("Hello World");
}
