pub struct Solution;

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut dp = vec![1; nums.len()];
        let mut prev = vec![None; nums.len()];
        let mut max = (0, None);
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    prev[i] = Some(j);
                }
            }
            if dp[i] > max.0 {
                max = (dp[i], Some(i));
            }
        }
        let mut answer = Vec::new();
        while let Some(i) = max.1 {
            answer.push(nums[i]);
            max.1 = prev[i];
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::largest_divisible_subset(vec![1, 2, 3]);
        ret.sort();
        assert_eq!(vec![1, 2], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::largest_divisible_subset(vec![1, 2, 4, 8]);
        ret.sort();
        assert_eq!(vec![1, 2, 4, 8], ret);
    }
}
