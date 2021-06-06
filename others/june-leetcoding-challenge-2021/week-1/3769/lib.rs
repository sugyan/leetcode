use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut hm = HashMap::new();
        for num in &nums {
            if !hm.contains_key(num) {
                let lt = *hm.get(&(num - 1)).unwrap_or(&0);
                let gt = *hm.get(&(num + 1)).unwrap_or(&0);
                hm.insert(num - lt, lt + gt + 1);
                hm.insert(num + gt, lt + gt + 1);
                hm.insert(*num, lt + gt + 1);
            }
        }
        *hm.values().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            9,
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
        );
    }
}
