use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return Vec::new();
        }
        let mut nums = nums;
        nums.sort();
        let mut answer = Vec::new();
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut j, mut k) = (i + 1, nums.len() - 1);
            while j < k {
                match (nums[i] + nums[j] + nums[k]).cmp(&0) {
                    Ordering::Less => j += 1,
                    Ordering::Greater => k -= 1,
                    Ordering::Equal => {
                        answer.push(vec![nums[i], nums[j], nums[k]]);
                        while j < k && nums[j] == nums[j + 1] {
                            j += 1;
                        }
                        while j < k && nums[k] == nums[k - 1] {
                            k -= 1;
                        }
                        j += 1;
                        k -= 1;
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
        let mut ret = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        ret.iter_mut().for_each(|v| v.sort());
        ret.sort();
        assert_eq!(vec![vec![-1, -1, 2], vec![-1, 0, 1]], ret);
    }

    #[test]
    fn example_2() {
        assert_eq!(Vec::<Vec<i32>>::new(), Solution::three_sum(vec![]));
    }

    #[test]
    fn example_3() {
        assert_eq!(Vec::<Vec<i32>>::new(), Solution::three_sum(vec![0]));
    }
}
