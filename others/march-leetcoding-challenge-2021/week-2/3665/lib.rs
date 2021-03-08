pub struct Solution;

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        if (0..s.len() / 2).all(|i| s.as_bytes()[i] == s.as_bytes()[s.len() - 1 - i]) {
            1
        } else {
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::remove_palindrome_sub(String::from("ababa")));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::remove_palindrome_sub(String::from("abb")));
    }

    #[test]
    fn example_3() {
        assert_eq!(2, Solution::remove_palindrome_sub(String::from("baabb")));
    }

    #[test]
    fn example_4() {
        assert_eq!(0, Solution::remove_palindrome_sub(String::from("")));
    }
}
