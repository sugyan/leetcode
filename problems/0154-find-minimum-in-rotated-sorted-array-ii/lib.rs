use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            match nums[mid].cmp(&nums[hi]) {
                Ordering::Less => hi = mid,
                Ordering::Equal => hi -= 1,
                Ordering::Greater => lo = mid + 1,
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
        assert_eq!(1, Solution::find_min(vec![1, 3, 5]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::find_min(vec![2, 2, 2, 0, 1]));
    }
}
