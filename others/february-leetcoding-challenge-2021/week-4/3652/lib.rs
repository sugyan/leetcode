pub struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r && nums[l] <= nums[l + 1] {
            l += 1;
        }
        if l == r {
            return 0;
        }
        while l < r && nums[r - 1] <= nums[r] {
            r -= 1;
        }
        let mut minmax = (std::i32::MAX, std::i32::MIN);
        for &num in nums.iter().skip(l).take(r - l + 1) {
            minmax.0 = std::cmp::min(minmax.0, num);
            minmax.1 = std::cmp::max(minmax.1, num);
        }
        while l > 0 && nums[l - 1] > minmax.0 {
            l -= 1;
        }
        while r < nums.len() - 1 && nums[r + 1] < minmax.1 {
            r += 1;
        }
        (r - l + 1) as i32
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
