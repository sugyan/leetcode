pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let v: Vec<char> = s.chars().collect();
        let (mut l, mut r) = (0, s.len() - 1);
        while l < r {
            while l < r && !v[l].is_alphanumeric() {
                l += 1;
            }
            while l < r && !v[r].is_alphanumeric() {
                r -= 1;
            }
            if !v[l].eq_ignore_ascii_case(&v[r]) {
                return false;
            }
            l += 1;
            if l < r {
                r -= 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::is_palindrome("race a car".to_string()));
    }
}
