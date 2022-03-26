use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len());
        while lo < hi {
            let mid = (lo + hi) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => lo = mid + 1,
                Ordering::Equal => return mid as i32,
                Ordering::Greater => hi = mid,
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
        assert_eq!(4, Solution::search(vec![-1, 0, 3, 5, 9, 12], 9));
    }

    #[test]
    fn example_2() {
        assert_eq!(-1, Solution::search(vec![-1, 0, 3, 5, 9, 12], 2));
    }
}
