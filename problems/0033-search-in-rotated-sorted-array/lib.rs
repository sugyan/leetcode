pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let m = l + (r - l) / 2;
            let mut n = nums[m];
            if n == target {
                return m as i32;
            }
            if (n < nums[0]) != (target < nums[0]) {
                n = if target < nums[0] {
                    i32::min_value()
                } else {
                    i32::max_value()
                }
            }
            if n > target {
                r = m;
            } else {
                l = m + 1;
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    }

    #[test]
    fn example_2() {
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    }
}
