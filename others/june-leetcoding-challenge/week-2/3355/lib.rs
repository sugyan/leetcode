pub struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let s: &[u8] = s.as_bytes();
        let mut i = 0;
        for b in t.as_bytes().iter() {
            if *b == s[i] {
                i += 1;
                if i == s.len() {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string())
        );
    }
}
