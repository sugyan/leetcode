pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len());
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::search_insert(vec![1, 3, 5, 6], 2));
    }

    #[test]
    fn example_3() {
        assert_eq!(4, Solution::search_insert(vec![1, 3, 5, 6], 7));
    }

    #[test]
    fn example_4() {
        assert_eq!(0, Solution::search_insert(vec![1, 3, 5, 6], 0));
    }

    #[test]
    fn example_5() {
        assert_eq!(0, Solution::search_insert(vec![1], 0));
    }
}
