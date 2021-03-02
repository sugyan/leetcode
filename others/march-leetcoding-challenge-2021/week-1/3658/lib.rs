pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut duplicated = 0;
        let mut xor = 0;
        for i in 0..nums.len() {
            let n = nums[i].abs();
            xor ^= n ^ ((i as i32) + 1);
            if nums[n as usize - 1] < 0 {
                duplicated = n;
            }
            nums[n as usize - 1] *= -1;
        }
        vec![duplicated, duplicated ^ xor]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![2, 3], Solution::find_error_nums(vec![1, 2, 2, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![1, 2], Solution::find_error_nums(vec![1, 1]));
    }
}
