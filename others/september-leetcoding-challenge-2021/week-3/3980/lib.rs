pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.bytes().collect::<Vec<_>>();
        let t = t.bytes().collect::<Vec<_>>();
        let mut dp = vec![vec![0; s.len() + 1]; t.len() + 1];
        (0..s.len()).for_each(|i| dp[0][i] = 1);
        for i in 0..t.len() {
            for j in 0..s.len() {
                dp[i + 1][j + 1] = dp[i + 1][j] + if s[j] == t[i] { dp[i][j] } else { 0 };
            }
        }
        dp[t.len()][s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::num_distinct(String::from("rabbbit"), String::from("rabbit"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            5,
            Solution::num_distinct(String::from("babgbag"), String::from("bag"))
        );
    }
}
