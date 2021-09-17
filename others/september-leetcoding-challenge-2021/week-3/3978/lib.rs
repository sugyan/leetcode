use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut hm = HashMap::<i32, u32>::new();
        nums1
            .iter()
            .for_each(|&num| *hm.entry(num).or_default() += 1);
        nums2
            .into_iter()
            .filter(|num| {
                if let Some(v) = hm.get_mut(num).filter(|c| **c > 0) {
                    *v -= 1;
                    true
                } else {
                    false
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]);
        ret.sort();
        assert_eq!(vec![2, 2], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        ret.sort();
        assert_eq!(vec![4, 9], ret);
    }
}
