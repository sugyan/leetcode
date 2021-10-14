pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![i32::MAX; n as usize + 1];
        dp[0] = 0;
        for i in 1..=n as usize {
            for j in (1..).take_while(|j| j * j <= i) {
                dp[i] = dp[i].min(dp[i - j * j] + 1);
            }
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::num_squares(12));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::num_squares(13));
    }
}
