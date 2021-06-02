pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                dp[i][j] = match (i == 0, j == 0) {
                    (true, true) => true,
                    (true, false) => dp[i][j - 1] && s2[j - 1] == s3[i + j - 1],
                    (false, true) => dp[i - 1][j] && s1[i - 1] == s3[i + j - 1],
                    (false, false) => {
                        (dp[i][j - 1] && s2[j - 1] == s3[i + j - 1])
                            || (dp[i - 1][j] && s1[i - 1] == s3[i + j - 1])
                    }
                }
            }
        }
        dp[s1.len()][s2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::is_interleave(
                String::from("aabcc"),
                String::from("dbbca"),
                String::from("aadbbcbcac")
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::is_interleave(
                String::from("aabcc"),
                String::from("dbbca"),
                String::from("aadbbbaccc")
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            true,
            Solution::is_interleave(String::from(""), String::from(""), String::from(""))
        );
    }
}
