use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut hm = HashMap::new();
        for num in nums {
            *hm.entry(num).or_insert(0) += 1;
        }
        hm.keys()
            .filter(|&key| hm.get(&(key + k)).map_or(false, |&v| k > 0 || v > 1))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::find_pairs(vec![3, 1, 4, 1, 5], 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::find_pairs(vec![1, 2, 3, 4, 5], 1));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::find_pairs(vec![1, 3, 1, 5, 4], 0));
    }
}
