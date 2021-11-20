pub struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if (mid % 2 == 0) == (nums[mid] != nums[mid + 1]) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        nums[lo]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            10,
            Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11])
        );
    }
}
