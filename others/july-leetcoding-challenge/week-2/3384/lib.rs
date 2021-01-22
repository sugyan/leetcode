pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return Vec::new();
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut answer: Vec<Vec<i32>> = Vec::new();
        let mut i = 0;
        while i < nums.len() - 2 {
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            while l < r {
                match (nums[i] + nums[l] + nums[r]).cmp(&0) {
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
                    std::cmp::Ordering::Less => l += 1,
                    std::cmp::Ordering::Greater => r -= 1,
                }
            }
            while i < nums.len() - 1 && nums[i] == nums[i + 1] {
                i += 1;
            }
            i += 1
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
        for v in ret.iter_mut() {
            v.sort_unstable();
        }
        ret.sort();
        assert_eq!(vec![vec![-1, -1, 2], vec![-1, 0, 1]], ret);
    }
}
