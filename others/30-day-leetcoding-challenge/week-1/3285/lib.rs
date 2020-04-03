pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut answer = std::i32::MIN;
        let mut sum = 0;
        for num in nums.iter() {
            sum += num;
            answer = std::cmp::max(answer, sum);
            if sum < 0 {
                sum = 0;
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
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
    }
}
