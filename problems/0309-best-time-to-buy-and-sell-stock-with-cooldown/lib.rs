pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut buy: Vec<i32> = vec![0; prices.len()];
        let mut sell: Vec<i32> = vec![0; prices.len()];
        let mut cooldown: Vec<i32> = vec![0; prices.len()];
        sell[0] = std::i32::MIN;
        buy[0] = -prices[0];
        for i in 1..prices.len() {
            buy[i] = std::cmp::max(buy[i - 1], cooldown[i - 1] - prices[i]);
            sell[i] = std::cmp::max(sell[i - 1], buy[i - 1] + prices[i]);
            cooldown[i] = std::cmp::max(cooldown[i - 1], sell[i - 1]);
        }
        std::cmp::max(sell[prices.len() - 1], cooldown[prices.len() - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::max_profit(vec![1, 2, 3, 0, 2]));
    }
}
