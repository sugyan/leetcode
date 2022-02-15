pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::single_number(vec![4, 1, 2, 1, 2]));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::single_number(vec![1]));
    }
}
