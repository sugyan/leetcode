pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![None; amount + 1];
        dp[0] = Some(0);
        for i in 1..=amount {
            dp[i] = coins
                .iter()
                .filter_map(|&j| {
                    let j = j as usize;
                    if j <= i {
                        dp[i - j].map(|n| n + 1)
                    } else {
                        None
                    }
                })
                .min();
        }
        dp[amount].unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::coin_change(vec![1, 2, 5], 11));
    }

    #[test]
    fn example_2() {
        assert_eq!(-1, Solution::coin_change(vec![2], 3));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::coin_change(vec![1], 0));
    }

    #[test]
    fn example_4() {
        assert_eq!(1, Solution::coin_change(vec![1], 1));
    }

    #[test]
    fn example_5() {
        assert_eq!(2, Solution::coin_change(vec![1], 2));
    }
}
