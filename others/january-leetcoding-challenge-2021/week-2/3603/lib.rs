pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut sum = nums.iter().sum::<i32>();
        if sum < x {
            return -1;
        }
        let (mut l, mut max) = (0, 0);
        for r in 0..=nums.len() {
            while l < r && sum < x {
                sum += nums[l];
                l += 1;
            }
            if sum == x {
                max = std::cmp::max(max, r - l + 1);
            }
            if r < nums.len() {
                sum -= nums[r];
            }
        }
        if max == 0 {
            -1
        } else {
            (nums.len() - max) as i32 + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::min_operations(vec![1, 1, 4, 2, 3], 5));
    }

    #[test]
    fn example_2() {
        assert_eq!(-1, Solution::min_operations(vec![5, 6, 7, 8, 9], 4));
    }

    #[test]
    fn example_3() {
        assert_eq!(5, Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10));
    }
}
