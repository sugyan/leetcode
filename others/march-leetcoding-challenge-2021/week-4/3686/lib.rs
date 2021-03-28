pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        (0..s.len())
            .map(|i| Self::count_parindrome(&s, i, i) + Self::count_parindrome(&s, i, i + 1))
            .sum()
    }
    fn count_parindrome(s: &[u8], i: usize, j: usize) -> i32 {
        let (mut i, mut j) = (i, j);
        let mut ret = 0;
        while j < s.len() && s[i] == s[j] {
            ret += 1;
            if i == 0 {
                break;
            }
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
