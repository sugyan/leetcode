pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let m = l + (r - l) / 2;
            match nums[m].cmp(&target) {
                std::cmp::Ordering::Equal => return m as i32,
                std::cmp::Ordering::Less => l = m + 1,
                std::cmp::Ordering::Greater => r = m,
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
