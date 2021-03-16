pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let dp = prices.iter().skip(1).fold((-prices[0], 0), |acc, &x| {
            (acc.0.max(acc.1 - x), acc.1.max(acc.0 + x - fee))
        });
        dp.0.max(dp.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(8, Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(6, Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3));
    }
}
