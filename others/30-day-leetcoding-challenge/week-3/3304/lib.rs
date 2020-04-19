pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() as i32 - 1);
        while l <= r {
            let m = (l + (r - l) / 2) as usize;
            if nums[m] == target {
                return m as i32;
            }
            if nums[m] < nums[l as usize] {
                if target <= nums[r as usize] && target > nums[m] {
                    l = m as i32 + 1
                } else {
                    r = m as i32 - 1
                }
            } else if nums[m] > nums[r as usize] {
                if target >= nums[l as usize] && target < nums[m] {
                    r = m as i32 - 1
                } else {
                    l = m as i32 + 1
                }
            } else if target > nums[m] {
                l = m as i32 + 1
            } else {
                r = m as i32 - 1
            }
        }
        -1
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
