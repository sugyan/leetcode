use std::collections::BTreeSet;

pub struct Solution {}

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if t < 0 {
            return false;
        }
        let k = k as usize;
        let t = t as i64;
        let mut bts: BTreeSet<i64> = BTreeSet::new();
        for i in 0..nums.len() {
            if i > k {
                bts.remove(&(nums[i - 1 - k] as i64));
            }
            if bts
                .range(nums[i] as i64 - t..=nums[i] as i64 + t)
                .next()
                .is_some()
            {
                return true;
            }
            bts.insert(nums[i] as i64);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            true,
            Solution::contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3)
        );
    }
}
