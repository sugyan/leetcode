pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(answer, len), &x| {
                if x == 0 {
                    (answer, 0)
                } else {
                    (answer.max(len + 1), len + 1)
                }
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
