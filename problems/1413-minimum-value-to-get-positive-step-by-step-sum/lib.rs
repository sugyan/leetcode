pub struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        1 - nums
            .iter()
            .scan(0, |state, &num| {
                *state += num;
                Some(*state)
            })
            .min()
            .unwrap()
            .min(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::min_start_value(vec![-3, 2, -3, 4, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::min_start_value(vec![1, 2]));
    }

    #[test]
    fn example_3() {
        assert_eq!(5, Solution::min_start_value(vec![1, -2, -3]));
    }
}
