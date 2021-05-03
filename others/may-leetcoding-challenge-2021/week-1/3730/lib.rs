pub struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .scan(0, |state, &x| {
                *state += x;
                Some(*state)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![1, 3, 6, 10], Solution::running_sum(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![1, 2, 3, 4, 5],
            Solution::running_sum(vec![1, 1, 1, 1, 1])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec![3, 4, 6, 16, 17],
            Solution::running_sum(vec![3, 1, 2, 10, 1])
        );
    }
}
