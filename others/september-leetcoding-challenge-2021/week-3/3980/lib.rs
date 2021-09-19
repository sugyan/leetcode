pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut dp = vec![0; t.len() + 1];
        dp[0] = 1;
        for s in s.bytes() {
            for (i, t) in t.bytes().enumerate().rev() {
                dp[i + 1] += if s == t { dp[i] } else { 0 };
            }
        }
        dp[t.len()]
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
