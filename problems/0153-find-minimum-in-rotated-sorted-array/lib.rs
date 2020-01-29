pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] > nums[r] {
                l = m + 1;
            } else {
                r = m;
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
        assert_eq!(1, Solution::find_min(vec![3, 4, 5, 1, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]));
    }
}
