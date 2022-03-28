pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo <= hi {
            let mid = (lo + hi) / 2;
            if nums[mid] == target {
                return true;
            }
            if nums[lo] == nums[mid] && nums[mid] == nums[hi] {
                lo += 1;
                hi -= hi.min(1);
                continue;
            }
            if nums[lo] <= nums[mid] {
                if nums[lo] <= target && nums[mid] > target {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else if nums[mid] < target && nums[hi] >= target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn example_1() {
        assert_eq!(true, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn example_2() {
        assert_eq!(false, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }
}
