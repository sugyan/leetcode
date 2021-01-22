pub struct Solution {}

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        let mut v: Vec<i32> = Vec::new();
        nums.sort_unstable();
        Solution::dfs(&mut answer, &nums, &mut v, 0);
        answer
    }
    fn dfs(answer: &mut Vec<Vec<i32>>, nums: &[i32], v: &mut Vec<i32>, i: usize) {
        answer.push(v.clone());
        for j in i..nums.len() {
            if j != i && j > 0 && nums[j] == nums[j - 1] {
                continue;
            }
            v.push(nums[j]);
            Solution::dfs(answer, nums, v, j + 1);
            v.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::subsets_with_dup(vec![1, 2, 2]);
        ret.sort();
        assert_eq!(
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2]
            ],
            ret
        );
    }
}
