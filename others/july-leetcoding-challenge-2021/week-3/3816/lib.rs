use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut answer = Vec::new();
        let len = nums.len();
        if len < 4 {
            return answer;
        }
        let mut nums = nums;
        nums.sort_unstable();
        for i in 0..len - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..len - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let sum_ij = nums[i] + nums[j];
                if sum_ij + nums[j + 1] + nums[j + 2] > target
                    || sum_ij + nums[len - 2] + nums[len - 1] < target
                {
                    continue;
                }
                let (mut k, mut l) = (j + 1, len - 1);
                while k < l {
                    if k > j + 1 && nums[k] == nums[k - 1] {
                        k += 1;
                        continue;
                    }
                    if l < len - 1 && nums[l] == nums[l + 1] {
                        l -= 1;
                        continue;
                    }
                    match (sum_ij + nums[k] + nums[l]).cmp(&target) {
                        Ordering::Less => k += 1,
                        Ordering::Equal => {
                            answer.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                            k += 1;
                            l -= 1;
                        }
                        Ordering::Greater => l -= 1,
                    }
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
