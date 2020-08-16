pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut dp: Vec<Vec<i32>> = vec![vec![0; prices.len()]; 3];
        for i in 1..3 {
            for j in 1..prices.len() {
                let mut max = 0;
                for k in 0..j {
                    max = std::cmp::max(max, prices[j] - prices[k] + dp[i - 1][k]);
                }
                dp[i][j] = std::cmp::max(dp[i][j - 1], max);
            }
        }
        dp[2][prices.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(6, Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }
}
