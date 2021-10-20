pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "blue is sky the",
            Solution::reverse_words(String::from("the sky is blue"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "world! hello",
            Solution::reverse_words(String::from("  hello world!  "))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "example good a",
            Solution::reverse_words(String::from("a good   example"))
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            "Alice Loves Bob",
            Solution::reverse_words(String::from("  Bob    Loves  Alice   "))
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            "bob like even not does Alice",
            Solution::reverse_words(String::from("Alice does not even like bob"))
        );
    }
}
