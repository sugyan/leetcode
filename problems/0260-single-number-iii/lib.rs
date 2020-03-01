use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut hs: HashSet<i32> = HashSet::new();
        for n in nums.iter() {
            if hs.contains(n) {
                hs.remove(n);
            } else {
                hs.insert(*n);
            }
        }
        hs.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::single_number(vec![1, 2, 1, 3, 2, 5]);
        ret.sort();
        assert_eq!(vec![3, 5], ret);
    }
}
