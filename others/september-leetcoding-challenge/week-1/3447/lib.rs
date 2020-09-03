pub struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.as_bytes();
        for i in 1..s.len() {
            if s.len() % i == 0 {
                let mut ok = true;
                for j in 1..s.len() / i {
                    if s[i * j..i * (j + 1)] != s[0..i] {
                        ok = false;
                        break;
                    }
                }
                if ok {
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
