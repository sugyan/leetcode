pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![None; amount as usize + 1];
        dp[0] = Some(0);
        for i in 0..=amount as usize {
            if let Some(min) = dp[i] {
                coins.iter().for_each(|&j| {
                    let j = j as usize;
                    if i + j <= amount as usize {
                        dp[i + j] = dp[i + j].map(|n| n.min(min + 1)).or(Some(min + 1))
                    }
                });
            }
        }
        dp[amount as usize].unwrap_or(-1)
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
