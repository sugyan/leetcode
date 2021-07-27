pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut answer = nums.iter().take(3).sum::<i32>();
        for i in 0..nums.len() {
            let (mut j, mut k) = (i + 1, nums.len() - 1);
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum > target {
                    k -= 1;
                } else {
                    j += 1;
                }
                if (sum - target).abs() < (answer - target).abs() {
                    answer = sum;
                }
            }
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
