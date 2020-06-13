use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut hm: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut nums: Vec<i32> = nums;
        nums.sort();
        let mut answer: Vec<i32> = Vec::new();
        for num in nums.iter() {
            let mut v: Vec<i32> = if let Some(e) = hm
                .iter()
                .filter(|&e| num % e.0 == 0)
                .max_by_key(|&e| e.1.len())
            {
                e.1.clone()
            } else {
                Vec::new()
            };
            v.push(*num);
            if v.len() > answer.len() {
                answer = v.clone();
            }
            hm.insert(*num, v);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::largest_divisible_subset(vec![1, 2, 3]);
        ret.sort();
        assert!(ret == vec![1, 2] || ret == vec![1, 3]);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::largest_divisible_subset(vec![1, 2, 4, 8]);
        ret.sort();
        assert_eq!(vec![1, 2, 4, 8], ret);
    }
}
