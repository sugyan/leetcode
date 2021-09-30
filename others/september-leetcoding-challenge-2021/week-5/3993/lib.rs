pub struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % k != 0 {
            return false;
        }
        Self::backtrack(&nums, 0, sum / k, 0, &mut vec![false; nums.len()])
    }
    fn backtrack(nums: &[i32], curr: i32, target: i32, i: usize, used: &mut Vec<bool>) -> bool {
        if used.iter().all(|&b| b) {
            return true;
        }
        for j in i..nums.len() {
            if used[j] || curr + nums[j] > target {
                continue;
            }
            let next = (curr + nums[j]) % target;
            used[j] = true;
            if Self::backtrack(nums, next, target, if next == 0 { 0 } else { j + 1 }, used) {
                return true;
            }
            used[j] = false;
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
