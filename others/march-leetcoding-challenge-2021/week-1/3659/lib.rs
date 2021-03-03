pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        (nums.len() * (nums.len() + 1) / 2) as i32 - nums.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::missing_number(vec![3, 0, 1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::missing_number(vec![0, 1]));
    }

    #[test]
    fn example_3() {
        assert_eq!(8, Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
    }

    #[test]
    fn example_4() {
        assert_eq!(1, Solution::missing_number(vec![0]));
    }
}
