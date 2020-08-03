pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let v: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|&c| c.is_alphanumeric())
            .collect();
        if v.is_empty() {
            return true;
        }
        let (mut l, mut r) = (0, v.len() - 1);
        while l < r {
            if v[l] != v[r] {
                return false;
            }
            l += 1;
            r -= 1;
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
