use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut counts = vec![HashMap::new(); nums.len()];
        let mut answer = 0;
        for i in 1..nums.len() {
            for j in 0..i {
                let delta = nums[i] as i64 - nums[j] as i64;
                let sum = *counts[j].get(&delta).unwrap_or(&0);
                let origin = *counts[i].get(&delta).unwrap_or(&0);
                counts[i].insert(delta, origin + sum + 1);
                answer += sum;
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
            7,
            Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            16,
            Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7])
        );
    }
}
