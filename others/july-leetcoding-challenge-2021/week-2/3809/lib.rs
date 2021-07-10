pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut dp = (
            1,
            match s[0] {
                '*' => 9,
                '0' => 0,
                _ => 1,
            },
        );
        for i in 1..s.len() {
            let n = if s[i] == '*' {
                dp.0 * match s[i - 1] {
                    '*' => 15,
                    '1' => 9,
                    '2' => 6,
                    _ => 0,
                } + dp.1 * 9
            } else {
                dp.0 * match s[i - 1] {
                    '*' if s[i] <= '6' => 2,
                    '2' if s[i] <= '6' => 1,
                    '*' | '1' => 1,
                    _ => 0,
                } + if s[i] == '0' { 0 } else { dp.1 }
            };
            dp = (dp.1, n % MOD);
        }
        dp.1 as i32
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
