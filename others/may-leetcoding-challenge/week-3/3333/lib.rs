pub struct Solution {}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut d1: [usize; 26] = [0; 26];
        let mut d2: [usize; 26] = [0; 26];
        for c in s1.chars() {
            d1[(c as u8 - b'a') as usize] += 1;
        }
        let s: &[u8] = s2.as_bytes();
        for (i, c) in s.iter().enumerate() {
            d2[(c - b'a') as usize] += 1;
            if i >= s1.len() {
                d2[(s[i - s1.len()] - b'a') as usize] -= 1;
            }
            if d1 == d2 {
                return true;
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
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string())
        );
    }
}
