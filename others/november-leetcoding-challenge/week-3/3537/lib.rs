pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            let m = l + (r - l) / 2;
            if nums[m] == target {
                return true;
            }
            if nums[l] == nums[m] && nums[m] == nums[r] {
                l += 1;
                if r > 0 {
                    r -= 1;
                }
            } else if nums[l] <= nums[m] {
                if nums[l] <= target && nums[m] > target {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else if nums[m] < target && nums[r] >= target {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }
}
