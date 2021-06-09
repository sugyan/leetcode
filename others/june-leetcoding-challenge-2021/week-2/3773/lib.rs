use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut vd = VecDeque::new();
        for i in (0..nums.len()).rev() {
            dp[i] = nums[i] + vd.front().map_or(0, |&f| dp[f]);
            while !vd.is_empty() {
                if let Some(&b) = vd.back() {
                    if dp[i] > dp[b] {
                        vd.pop_back();
                    } else {
                        break;
                    }
                }
            }
            vd.push_back(i);
            if let Some(&f) = vd.front() {
                if f == i + k as usize {
                    vd.pop_front();
                }
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(7, Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(17, Solution::max_result(vec![10, -5, -2, 4, 0, 3], 3));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            0,
            Solution::max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2)
        );
    }
}
