use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut hs: HashMap<i32, usize> = HashMap::new();
        for num in nums.iter() {
            if let Some(n) = hs.get_mut(&num) {
                *n += 1;
            } else {
                hs.insert(*num, 0);
            }
        }
        hs.iter()
            .filter(|e| *e.1 >= nums.len() / 3)
            .map(|e| *e.0)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::majority_element(vec![3, 2, 3]);
        ret.sort();
        assert_eq!(vec![3], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::majority_element(vec![1, 1, 1, 3, 3, 2, 2, 2]);
        ret.sort();
        assert_eq!(vec![1, 2], ret);
    }
}
