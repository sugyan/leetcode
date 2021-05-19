pub struct Solution;

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums.iter().map(|n| (nums[nums.len() / 2] - n).abs()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::min_moves2(vec![1, 2, 3]));
    }

    #[test]
    fn example_2() {
        assert_eq!(16, Solution::min_moves2(vec![1, 10, 2, 9]));
    }
}
