pub struct Solution {}

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        if k >= prices.len() / 2 {
            (1..prices.len())
                .map(|i| prices[i] - prices[i - 1])
                .filter(|&x| x > 0)
                .sum()
        } else {
            let mut buy: Vec<i32> = vec![std::i32::MIN; k + 1];
            let mut sell: Vec<i32> = vec![0; k + 1];
            for &price in prices.iter() {
                for i in 0..k {
                    buy[i + 1] = std::cmp::max(buy[i + 1], sell[i] - price);
                    sell[i + 1] = std::cmp::max(sell[i + 1], buy[i + 1] + price);
                }
            }
            sell[k]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::max_profit(2, vec![2, 4, 1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(7, Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]));
    }
}
