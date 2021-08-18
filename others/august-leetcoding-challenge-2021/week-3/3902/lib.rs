pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes().iter().map(|&u| u - b'0').collect::<Vec<_>>();
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;
        for i in 0..s.len() {
            if s[i] > 0 {
                dp[i + 1] += dp[i];
            }
            if i > 0 && matches!((s[i - 1], s[i]), (1, _) | (2, 0..=6)) {
                dp[i + 1] += dp[i - 1];
            }
        }
        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::num_decodings(String::from("12")));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::num_decodings(String::from("226")));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::num_decodings(String::from("0")));
    }

    #[test]
    fn example_4() {
        assert_eq!(0, Solution::num_decodings(String::from("06")));
    }
}
