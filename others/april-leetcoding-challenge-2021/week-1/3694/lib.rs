pub struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];
        for s in &strs {
            let counts = s.as_bytes().iter().fold((0, 0), |acc, c| match c {
                b'0' => (acc.0 + 1, acc.1),
                b'1' => (acc.0, acc.1 + 1),
                _ => acc,
            });
            for i in (counts.0..=m as usize).rev() {
                for j in (counts.1..=n as usize).rev() {
                    dp[i][j] = dp[i][j].max(dp[i - counts.0][j - counts.1] + 1);
                }
            }
        }
        dp[m as usize][n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            4,
            Solution::find_max_form(
                vec!["10", "0001", "111001", "1", "0"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                5,
                3
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            2,
            Solution::find_max_form(
                vec!["10", "0", "1"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                1,
                1
            )
        );
    }
}
