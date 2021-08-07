pub struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let mut p = vec![vec![false; s.len()]; s.len()];
        let mut dp = vec![0; s.len()];
        for i in 0..s.len() {
            let mut min = i as i32;
            for j in 0..=i {
                if (j + 1 >= i || p[j + 1][i - 1]) && s[j] == s[i] {
                    p[j][i] = true;
                    min = if j == 0 { 0 } else { min.min(dp[j - 1] + 1) };
                }
            }
            dp[i] = min;
        }
        dp[s.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::min_cut(String::from("aab")));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::min_cut(String::from("a")));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::min_cut(String::from("ab")));
    }
}
