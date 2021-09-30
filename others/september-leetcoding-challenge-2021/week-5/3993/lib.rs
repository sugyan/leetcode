use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % k != 0 {
            return false;
        }
        Self::backtrack(&nums, 0, sum / k, &mut 0, &mut HashSet::new())
    }
    fn backtrack(
        nums: &[i32],
        sum: i32,
        target: i32,
        used: &mut u32,
        memo: &mut HashSet<u32>,
    ) -> bool {
        if memo.contains(&used) {
            return false;
        }
        if used.count_ones() == nums.len() as u32 {
            return true;
        }
        for i in 0..nums.len() {
            let mask = 1 << i;
            if (*used & mask == 0) && sum + nums[i] <= target {
                *used |= mask;
                if Self::backtrack(nums, (sum + nums[i]) % target, target, used, memo) {
                    return true;
                }
                *used &= !mask;
            }
        }
        memo.insert(*used);
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
            Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3)
        );
    }
}
