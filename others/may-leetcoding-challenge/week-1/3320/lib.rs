pub struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut d: [usize; 26] = [0; 26];
        for c in s.chars().map(|c| (c as u8 - b'a') as usize) {
            d[c] += 1;
        }
        for (i, c) in s.chars().map(|c| (c as u8 - b'a') as usize).enumerate() {
            if d[c] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(0, Solution::first_uniq_char("leetcode".to_string()))
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::first_uniq_char("loveleetcode".to_string()))
    }
}
