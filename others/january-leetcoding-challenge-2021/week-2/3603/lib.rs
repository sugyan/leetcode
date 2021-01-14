pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut sum = vec![0; nums.len() + 1];
        for i in 1..sum.len() {
            sum[i] += sum[i - 1] + nums[i - 1];
        }
        let target = sum[sum.len() - 1] - x;
        if target < 0 {
            return -1;
        }
        let mut l = 0;
        let mut max = 0;
        for r in 0..sum.len() {
            while l < r && sum[r] - sum[l] > target {
                l += 1;
            }
            if sum[r] - sum[l] == target {
                max = std::cmp::max(max, r - l + 1);
            }
        }
        if max == 0 {
            -1
        } else {
            (sum.len() - max) as i32
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
