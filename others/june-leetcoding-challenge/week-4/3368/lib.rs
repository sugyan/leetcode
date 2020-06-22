pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        (0..32)
            .map(|i| 1 << i)
            .filter(|d| nums.iter().filter(|&n| n & d != 0).count() % 3 != 0)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::single_number(vec![2, 2, 3, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(99, Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]));
    }
}
