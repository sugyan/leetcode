use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut hs = HashSet::new();
        let (mut answer, mut sum) = (0, 0);
        let mut lo = 0;
        for hi in 0..nums.len() {
            while hs.contains(&nums[hi]) {
                sum -= nums[lo];
                hs.remove(&nums[lo]);
                lo += 1
            }
            sum += nums[hi];
            answer = answer.max(sum);
            hs.insert(nums[hi]);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(17, Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            8,
            Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5])
        );
    }
}
