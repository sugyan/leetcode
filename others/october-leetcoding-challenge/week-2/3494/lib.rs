pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut answer = 0;
        {
            let mut prev = (0, 0);
            for &num in nums.iter().skip(1) {
                prev = (prev.1, std::cmp::max(prev.1, prev.0 + num));
            }
            answer = std::cmp::max(answer, prev.1);
        }
        {
            let mut prev = (0, 0);
            for &num in nums.iter().rev().skip(1) {
                prev = (prev.1, std::cmp::max(prev.1, prev.0 + num));
            }
            answer = std::cmp::max(answer, prev.1);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::rob(vec![2, 3, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::rob(vec![0]));
    }
}
