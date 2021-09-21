pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |acc, &x| {
                let len = if x == 0 { 0 } else { acc.1 + 1 };
                (acc.0.max(len), len)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            2,
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1])
        );
    }
}
