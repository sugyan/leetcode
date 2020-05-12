pub struct Solution {}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = l + (r - l) / 2;
            if (m % 2 == 0) == (nums[m] != nums[m + 1]) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        nums[l]
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
