use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return Vec::new();
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut answer = Vec::new();
        let mut i = 0;
        while i < nums.len() - 3 {
            let mut j = i + 1;
            while j < nums.len() - 2 {
                let (mut k, mut l) = (j + 1, nums.len() - 1);
                while k < l {
                    match (nums[i] + nums[j] + nums[k] + nums[l]).cmp(&target) {
                        Ordering::Less => k += 1,
                        Ordering::Equal => {
                            answer.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                            while k + 1 < nums.len() && nums[k + 1] == nums[k] {
                                k += 1;
                            }
                            k += 1;
                        }
                        Ordering::Greater => l -= 1,
                    }
                }
                while j + 1 < nums.len() && nums[j + 1] == nums[j] {
                    j += 1;
                }
                j += 1;
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
        assert_eq!(
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![vec![2, 2, 2, 2]],
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8)
        );
    }
}
