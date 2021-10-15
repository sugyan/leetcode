pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = vec![0; prices.len()];
        let mut sell = vec![0; prices.len()];
        let mut cooldown = vec![0; prices.len()];
        buy[0] = -prices[0];
        sell[0] = i32::MIN;
        for i in 1..prices.len() {
            buy[i] = buy[i - 1].max(cooldown[i - 1] - prices[i]);
            sell[i] = sell[i - 1].max(buy[i - 1] + prices[i]);
            cooldown[i] = cooldown[i - 1].max(sell[i - 1]);
        }
        sell[prices.len() - 1].max(cooldown[prices.len() - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::max_profit(vec![1, 2, 3, 0, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::max_profit(vec![1]));
    }
}
