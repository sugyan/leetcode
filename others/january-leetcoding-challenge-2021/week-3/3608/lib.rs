use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut hm = HashMap::new();
        for &num in nums.iter() {
            *hm.entry(num).or_insert(0) += 1;
        }
        let mut answer = 0;
        for (&key, &value) in hm.iter().filter(|(&key, _)| key * 2 <= k) {
            if key * 2 == k {
                answer += value / 2;
            } else {
                if let Some(&v) = hm.get(&(k - key)) {
                    answer += std::cmp::min(value, v);
                }
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
        assert_eq!(2, Solution::max_operations(vec![1, 2, 3, 4], 5));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::max_operations(vec![3, 1, 3, 4, 3], 6));
    }
}
