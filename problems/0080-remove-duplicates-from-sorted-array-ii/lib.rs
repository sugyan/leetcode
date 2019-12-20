pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut idx, mut count, mut prev) = (0, 0, std::i32::MAX);
        for i in 0..nums.len() {
            count = if nums[i] == prev { count + 1 } else { 1 };
            prev = nums[i];
            if count <= 2 {
                nums.swap(i, idx);
                idx += 1;
            }
        }
        return idx as i32;
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
