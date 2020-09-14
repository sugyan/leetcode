pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            _ => {
                let mut max = (nums[0], std::cmp::max(nums[0], nums[1]));
                for &num in nums.iter().skip(2) {
                    max = (max.1, std::cmp::max(max.0 + num, max.1));
                }
                std::cmp::max(max.0, max.1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
    }
}
