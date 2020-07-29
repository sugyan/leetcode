pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let (mut buy, mut sell, mut cooldown) = (
            Vec::with_capacity(prices.len()),
            Vec::with_capacity(prices.len()),
            Vec::with_capacity(prices.len()),
        );
        buy.push(-prices[0]);
        sell.push(std::i32::MIN);
        cooldown.push(0);
        for i in 1..prices.len() {
            buy.push(std::cmp::max(buy[i - 1], cooldown[i - 1] - prices[i]));
            sell.push(std::cmp::max(sell[i - 1], buy[i - 1] + prices[i]));
            cooldown.push(std::cmp::max(cooldown[i - 1], sell[i - 1]));
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
