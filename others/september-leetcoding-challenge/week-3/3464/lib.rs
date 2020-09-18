pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut min, mut answer) = (std::i32::MAX, 0);
        for &price in prices.iter() {
            min = std::cmp::min(min, price);
            answer = std::cmp::max(answer, price - min);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }
}
