pub struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; n as usize + 1];
        for i in 1..=n as usize {
            if let Some(min) = (1..=(i as f32).sqrt() as usize)
                .map(|j| dp[i - j * j] + 1)
                .min()
            {
                dp[i] = min;
            }
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::num_squares(12));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::num_squares(13));
    }
}
