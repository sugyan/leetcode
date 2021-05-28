use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut hm = HashMap::new();
        let mut sums = vec![0; nums.len() + 1];
        let mut j = 0;
        let mut answer = 0;
        for (i, num) in nums.iter().enumerate() {
            sums[i + 1] = sums[i] + num;
            if let Some(&k) = hm.get(num) {
                j = j.max(k);
            }
            hm.insert(num, i + 1);
            answer = answer.max(sums[i + 1] - sums[j]);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(17, Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            8,
            Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5])
        );
    }
}
