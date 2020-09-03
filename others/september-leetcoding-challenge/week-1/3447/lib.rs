pub struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        (s.clone() + &s)[1..s.len() * 2 - 1].contains(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::repeated_substring_pattern(String::from("abab"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::repeated_substring_pattern(String::from("aba"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            true,
            Solution::repeated_substring_pattern(String::from("abcabcabcabc"))
        );
    }
}
