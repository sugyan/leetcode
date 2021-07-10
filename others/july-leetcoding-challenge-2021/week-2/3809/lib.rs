pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;
        for i in 0..s.len() {
            dp[i + 1] += dp[i]
                * match s[i] {
                    b'*' => 9,
                    b'0' => 0,
                    _ => 1,
                };
            if i > 0 {
                dp[i + 1] += dp[i - 1]
                    * match (s[i - 1], s[i]) {
                        (b'*', b'*') => 15,
                        (b'*', b'0'..=b'6') => 2,
                        (b'*', _) => 1,
                        (b'1', b'*') => 9,
                        (b'1', _) => 1,
                        (b'2', b'*') => 6,
                        (b'2', b'0'..=b'6') => 1,
                        _ => 0,
                    };
            }
            dp[i + 1] %= MOD;
        }
        println!("{:?}", dp);
        dp[s.len()] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(9, Solution::num_decodings(String::from("*")));
    }

    #[test]
    fn example_2() {
        assert_eq!(18, Solution::num_decodings(String::from("1*")));
    }

    #[test]
    fn example_3() {
        assert_eq!(15, Solution::num_decodings(String::from("2*")));
    }
}
