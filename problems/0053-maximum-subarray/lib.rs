pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter()
            .scan(0, |state, &x| {
                let sum = *state + x;
                *state = sum.max(0);
                Some(sum)
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::max_sub_array(vec![1]));
    }

    #[test]
    fn example_3() {
        assert_eq!(23, Solution::max_sub_array(vec![5, 4, -1, 7, 8]));
    }
}
