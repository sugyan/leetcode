pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if (i == 0 || nums[i - 1] < nums[i]) && (i == nums.len() - 1 || nums[i] > nums[i + 1]) {
                return i as i32;
            }
        }
        unreachable!()
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
        assert_eq!(1, Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]));
    }
}
