pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        if k > n * (n - 1) / 2 {
            return 0;
        }
        let mut dp = vec![0; k as usize + 1];
        dp[0] = 1;
        for i in 1..=n as usize {
            let mut row = vec![0; k as usize + 1];
            let mut sum = 0_i32;
            for j in 0..=k as usize {
                sum += dp[j];
                if j >= i {
                    sum -= dp[j - i];
                }
                sum = sum.rem_euclid(MOD);
                row[j] = sum;
            }
            dp = row;
        }
        dp[k as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::k_inverse_pairs(3, 0));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::k_inverse_pairs(3, 1));
    }
}
