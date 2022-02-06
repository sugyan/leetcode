pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 0;
        for i in 0..nums.len() {
            if idx < 2 || nums[i] > nums[idx - 2] {
                nums[idx] = nums[i];
                idx += 1;
            }
        }
        idx as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let ret = Solution::remove_duplicates(&mut nums);
        assert_eq!(vec![1, 1, 2, 2, 3], &nums[0..ret as usize]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let ret = Solution::remove_duplicates(&mut nums);
        assert_eq!(vec![0, 0, 1, 1, 2, 3, 3], &nums[0..ret as usize]);
    }
}
