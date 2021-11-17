pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![1; n as usize];
        for _ in 1..m {
            for j in 1..n as usize {
                dp[j] += dp[j - 1];
            }
        }
        dp[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(28, Solution::unique_paths(3, 7));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::unique_paths(3, 2));
    }

    #[test]
    fn example_3() {
        assert_eq!(28, Solution::unique_paths(7, 3));
    }

    #[test]
    fn example_4() {
        assert_eq!(6, Solution::unique_paths(3, 3));
    }
}
