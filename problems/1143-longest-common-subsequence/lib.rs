pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        for (i, c1) in text1.chars().enumerate() {
            for (j, c2) in text2.chars().enumerate() {
                dp[i + 1][j + 1] = if c1 == c2 {
                    dp[i][j] + 1
                } else {
                    dp[i][j + 1].max(dp[i + 1][j])
                }
            }
        }
        dp[text1.len()][text2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::longest_common_subsequence(String::from("abcde"), String::from("ace"))
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            3,
            Solution::longest_common_subsequence(String::from("abc"), String::from("abc"))
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            0,
            Solution::longest_common_subsequence(String::from("abc"), String::from("def"))
        )
    }
}
