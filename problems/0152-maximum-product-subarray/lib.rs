pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut answer = std::i32::MIN;
        for i in 0..nums.len() {
            let mut p = 1;
            for j in nums[i..nums.len()].iter() {
                p *= j;
                answer = std::cmp::max(answer, p);
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(6, Solution::max_product(vec![2, 3, -2, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::max_product(vec![-2, 0, -1]));
    }
}
