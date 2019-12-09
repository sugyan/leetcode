pub struct Solution {}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        nums.sort();
        Solution::helper(nums, 0, &mut answer);
        return answer;
    }

    fn helper(nums: Vec<i32>, i: usize, answer: &mut Vec<Vec<i32>>) {
        if i == nums.len() - 1 {
            answer.push(nums.clone());
        } else {
            let mut nums = nums;
            for j in i..nums.len() {
                if j > i && nums[j] == nums[i] {
                    continue;
                }
                nums.swap(i, j);
                Solution::helper(nums.clone(), i + 1, answer);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::permute_unique(vec![1, 1, 2]);
        ret.sort();
        assert_eq!(vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]], ret);
    }
}
