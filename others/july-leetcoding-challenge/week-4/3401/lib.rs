pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = l + (r - l) / 2;
            match nums[m].cmp(&nums[r]) {
                std::cmp::Ordering::Less => r = m,
                std::cmp::Ordering::Equal => r -= 1,
                std::cmp::Ordering::Greater => l = m + 1,
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
        assert_eq!(1, Solution::find_min(vec![1, 3, 5]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::find_min(vec![2, 2, 2, 0, 1]));
    }
}
