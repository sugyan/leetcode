pub struct Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.concat() == word2.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::array_strings_are_equal(
                vec![String::from("ab"), String::from("c")],
                vec![String::from("a"), String::from("bc")]
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::array_strings_are_equal(
                vec![String::from("a"), String::from("cb")],
                vec![String::from("ab"), String::from("c")]
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            true,
            Solution::array_strings_are_equal(
                vec![String::from("abc"), String::from("d"), String::from("defg")],
                vec![String::from("abcddefg")]
            )
        );
    }
}
