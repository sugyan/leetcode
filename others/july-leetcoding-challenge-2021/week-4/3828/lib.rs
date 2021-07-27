use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut answer = nums.iter().take(3).sum::<i32>();
        let mut i = 0;
        while i < nums.len() - 2 {
            let (mut j, mut k) = (i + 1, nums.len() - 1);
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                match sum.cmp(&target) {
                    Ordering::Less => j += 1,
                    Ordering::Equal => return sum,
                    Ordering::Greater => k -= 1,
                }
                if (sum - target).abs() < (answer - target).abs() {
                    answer = sum;
                }
            }
            while i + 1 < nums.len() && nums[i + 1] == nums[i] {
                i += 1;
            }
            i += 1;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    }
}
