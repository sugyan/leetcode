pub struct Solution {}

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![0; amount as usize + 1];
        dp[0] = 1;
        for i in 0..coins.len() {
            for j in coins[i]..=amount {
                dp[j as usize] += dp[(j - coins[i]) as usize];
            }
        }
        dp[amount as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::change(5, vec![1, 2, 5]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::change(3, vec![2]));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::change(10, vec![10]));
    }
}
