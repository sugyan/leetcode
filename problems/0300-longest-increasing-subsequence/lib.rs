pub struct Solution {}

impl Solution {
    // pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    //     let mut dp: Vec<i32> = vec![1; nums.len()];
    //     for i in (0..nums.len()).rev() {
    //         for j in i + 1..nums.len() {
    //             if nums[i] < nums[j] {
    //                 dp[i] = std::cmp::max(dp[i], dp[j] + 1);
    //             }
    //         }
    //     }
    //     *dp.iter().max().unwrap()
    // }
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![1; nums.len()];
        for i in (0..nums.len()).rev() {
            for j in i + 1..nums.len() {
                if nums[i] < nums[j] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1);
                }
            }
        }
        if let Some(max) = dp.iter().max() {
            *max
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
    }
}
