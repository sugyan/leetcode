pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut answer = Vec::new();
        let mut prev = None;
        for i in 0..nums.len() {
            if let Some(p) = prev {
                if p == nums[i] {
                    continue;
                }
            }
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            while l < r {
                match nums[i].cmp(&-(nums[l] + nums[r])) {
                    std::cmp::Ordering::Less => {
                        l += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        r -= 1;
                    }
                    std::cmp::Ordering::Equal => {
                        answer.push(vec![nums[i], nums[l], nums[r]]);
                        while l < r && nums[l + 1] == nums[l] {
                            l += 1;
                        }
                        while l < r && nums[r - 1] == nums[r] {
                            r -= 1;
                        }
                        l += 1;
                        r -= 1;
                    }
                }
            }
            prev = Some(nums[i]);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        ret.sort();
        assert_eq!(vec![vec![-1, -1, 2], vec![-1, 0, 1]], ret);
    }
}
