pub struct Solution;

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        let mut answer = 0;
        for i in (0..nums1.len()).rev() {
            for j in (0..nums2.len()).rev() {
                if nums1[i] == nums2[j] {
                    dp[i][j] = dp[i + 1][j + 1] + 1;
                    answer = answer.max(dp[i][j]);
                };
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
        assert_eq!(
            3,
            Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7])
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            5,
            Solution::find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0])
        )
    }
}
