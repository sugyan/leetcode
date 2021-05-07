pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let s1 = word1.as_bytes();
        let s2 = word2.as_bytes();
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        for (i, b1) in s1.iter().enumerate() {
            for (j, b2) in s2.iter().enumerate() {
                dp[i + 1][j + 1] = if b1 == b2 {
                    dp[i + 1][j + 1].max(dp[i][j] + 1)
                } else {
                    dp[i + 1][j].max(dp[i][j + 1])
                }
            }
        }
        (s1.len() + s2.len() - 2 * dp[s1.len()][s2.len()]) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::min_distance(String::from("sea"), String::from("eat"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            4,
            Solution::min_distance(String::from("leetcode"), String::from("etco"))
        );
    }
}
