pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.trim();
        for (i, c) in s.chars().rev().enumerate() {
            if c == ' ' {
                return i as i32;
            }
        }
        s.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            5,
            Solution::length_of_last_word(String::from("Hello World"))
        );
    }
}
