pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![1; nums.len()];
        let (mut lo, mut hi) = (1, 1);
        for i in 1..nums.len() {
            lo *= nums[i - 1];
            answer[i] *= lo;
            hi *= nums[nums.len() - i];
            answer[nums.len() - i - 1] *= hi;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![0, 0, 9, 0, 0],
            Solution::product_except_self(vec![-1, 1, 0, -3, 3])
        );
    }
}
