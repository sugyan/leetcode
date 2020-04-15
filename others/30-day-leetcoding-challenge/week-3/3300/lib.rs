pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut answer: Vec<i32> = vec![1; nums.len()];
        let mut n = 1;
        for i in 0..nums.len() {
            answer[i] *= n;
            n *= nums[i];
        }
        n = 1;
        for i in (0..nums.len()).rev() {
            answer[i] *= n;
            n *= nums[i];
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
}
