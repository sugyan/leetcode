pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        (0..s.len())
            .map(|i| Self::count_parindrome(&s, i, i) + Self::count_parindrome(&s, i, i + 1))
            .sum()
    }
    fn count_parindrome(s: &[u8], i: usize, j: usize) -> i32 {
        if j == s.len() || s[i] != s[j] {
            return 0;
        }
        let (mut i, mut j) = (i, j);
        let mut ret = 1;
        while i > 0 && j < s.len() - 1 && s[i - 1] == s[j + 1] {
            ret += 1;
            i -= 1;
            j += 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::count_substrings(String::from("abc")));
    }

    #[test]
    fn example_2() {
        assert_eq!(6, Solution::count_substrings(String::from("aaa")));
    }
}
