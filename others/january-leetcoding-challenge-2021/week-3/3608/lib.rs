use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut hm = HashMap::new();
        let mut answer = 0;
        for &num in nums.iter() {
            if let Some(v) = hm.get_mut(&(k - num)) {
                answer += 1;
                *v -= 1;
                if *v == 0 {
                    hm.remove(&(k - num));
                }
            } else {
                *hm.entry(num).or_insert(0) += 1;
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
