pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        prices
            .iter()
            .fold((None, 0), |(hold, cash): (Option<i32>, _), &price| {
                (
                    hold.map(|h| h.max(cash - price)).or(Some(-price)),
                    hold.map_or(0, |h| cash.max(h + price - fee)),
                )
            })
            .1
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
