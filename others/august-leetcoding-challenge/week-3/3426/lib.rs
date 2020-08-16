pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut dp = [0; 3];
        let mut min = [prices[0]; 3];
        for price in prices.iter().skip(1) {
            for i in 1..3 {
                min[i] = std::cmp::min(min[i], price - dp[i - 1]);
                dp[i] = std::cmp::max(dp[i], price - min[i]);
            }
        }
        dp[2]
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
