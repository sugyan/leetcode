pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid] > nums[mid + 1] {
                hi = mid;
            } else {
                lo = mid + 1;
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
        assert_eq!(2, Solution::find_peak_element(vec![1, 2, 3, 1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(5, Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]));
    }
}
