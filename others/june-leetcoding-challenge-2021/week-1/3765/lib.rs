pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let mut dp = vec![false; s2.len() + 1];
        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                let u = if i + j > 0 { s3[i + j - 1] } else { 0 };
                dp[j] = match (i, j) {
                    (0, 0) => true,
                    (0, _) => dp[j - 1] && s2[j - 1] == u,
                    (_, 0) => dp[j] && s1[i - 1] == u,
                    _ => (dp[j - 1] && s2[j - 1] == u) || (dp[j] && s1[i - 1] == u),
                }
            }
        }
        dp[s2.len()]
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
