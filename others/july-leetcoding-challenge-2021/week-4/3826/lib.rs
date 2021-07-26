pub struct Solution;

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let mut dp = vec![0; 32];
        dp[0] = 1;
        dp[1] = 2;
        (2..32).for_each(|i| dp[i] = dp[i - 1] + dp[i - 2]);
        let mut answer = 1;
        for i in (0..31).rev() {
            if n & (1 << i) != 0 {
                answer += dp[i];
                if n & (1 << (i + 1)) != 0 {
                    answer -= 1;
                    break;
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::find_integers(5));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::find_integers(1));
    }

    #[test]
    fn example_3() {
        assert_eq!(3, Solution::find_integers(2));
    }
}
