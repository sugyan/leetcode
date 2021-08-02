use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm = HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            if let Some(&j) = hm.get(&(target - num)) {
                return [j as i32, i as i32].to_vec();
            }
            hm.insert(num, i);
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }

    #[test]
    fn example_3() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
    }
}
