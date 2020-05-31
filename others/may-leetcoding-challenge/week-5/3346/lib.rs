pub struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let v1: &[u8] = word1.as_bytes();
        let v2: &[u8] = word2.as_bytes();
        let mut dp: Vec<Vec<usize>> = vec![vec![0; v2.len() + 1]; v1.len() + 1];
        for i in 0..v1.len() {
            dp[i + 1][0] = i + 1;
        }
        for j in 0..v2.len() {
            dp[0][j + 1] = j + 1;
        }
        for (i, b1) in v1.iter().enumerate() {
            for (j, b2) in v2.iter().enumerate() {
                dp[i + 1][j + 1] = if *b1 == *b2 {
                    dp[i][j]
                } else {
                    1 + std::cmp::min(dp[i][j], std::cmp::min(dp[i + 1][j], dp[i][j + 1]))
                }
            }
        }
        dp[word1.len()][word2.len()] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::min_distance("horse".to_string(), "ros".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            5,
            Solution::min_distance("intention".to_string(), "execution".to_string())
        );
    }
}
