pub struct Solution {}

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums: Vec<i32> = nums;
        nums.sort();
        let mut dp: Vec<usize> = vec![1; nums.len()];
        let mut prev: Vec<Option<usize>> = vec![None; nums.len()];
        let (mut max, mut pos): (usize, Option<usize>) = (0, None);
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && dp[j] >= dp[i] {
                    dp[i] = dp[j] + 1;
                    prev[i] = Some(j);
                }
            }
            if dp[i] > max {
                max = dp[i];
                pos = Some(i);
            }
        }
        let mut answer: Vec<i32> = Vec::new();
        while let Some(p) = pos {
            answer.push(nums[p]);
            pos = prev[p];
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
        assert!(ret == vec![1, 2] || ret == vec![1, 3]);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::largest_divisible_subset(vec![1, 2, 4, 8]);
        ret.sort();
        assert_eq!(vec![1, 2, 4, 8], ret);
    }
}
