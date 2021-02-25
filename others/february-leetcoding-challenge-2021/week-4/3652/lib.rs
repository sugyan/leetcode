pub struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (nums.len() - 1, 0);
        let mut minmax = (nums[l], nums[r]);
        for i in (0..nums.len() - 1).rev() {
            if nums[i] <= minmax.0 && nums[i] <= nums[i + 1] {
                minmax.0 = nums[i];
            } else {
                l = i;
            }
        }
        for i in 1..nums.len() {
            if nums[i] >= minmax.1 && nums[i] >= nums[i - 1] {
                minmax.1 = nums[i];
            } else {
                r = i;
            }
        }
        if l >= r {
            0
        } else {
            (r - l + 1) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            5,
            Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::find_unsorted_subarray(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::find_unsorted_subarray(vec![1]));
    }
}
