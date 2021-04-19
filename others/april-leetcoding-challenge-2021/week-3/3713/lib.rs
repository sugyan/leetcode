pub struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 1..=target as usize {
            for &n in &nums {
                if i >= n as usize {
                    dp[i] += dp[i - n as usize];
                }
            }
        }
        dp[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(7, Solution::combination_sum4(vec![1, 2, 3], 4));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::combination_sum4(vec![9], 3));
    }
}
