pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let s2 = word2.as_bytes();
        let mut dp = vec![0; s2.len() + 1];
        for b1 in word1.as_bytes() {
            let mut prev = dp[0];
            for (j, b2) in s2.iter().enumerate() {
                let val = dp[j + 1].max(if b1 == b2 { prev + 1 } else { dp[j] });
                prev = dp[j + 1];
                dp[j + 1] = val;
            }
        }
        (word1.len() + word2.len() - 2 * dp[s2.len()]) as i32
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
