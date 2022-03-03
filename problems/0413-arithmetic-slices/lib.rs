pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        nums.windows(3)
            .scan(0, |n, w| {
                *n = if w[0] - w[1] == w[1] - w[2] {
                    *n + 1
                } else {
                    0
                };
                Some(*n)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::number_of_arithmetic_slices(vec![1]));
    }
}
