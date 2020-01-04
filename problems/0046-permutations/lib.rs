pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer = Vec::new();
        let mut v = Vec::new();
        Solution::helper(&nums, &mut v, &mut answer);
        answer
    }
    fn helper(nums: &[i32], v: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>) {
        if v.len() == nums.len() {
            answer.push(v.clone());
        } else {
            for num in nums {
                if !v.contains(&num) {
                    v.push(*num);
                    Solution::helper(nums, v, answer);
                    v.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::permute(vec![1, 2, 3]);
        ret.sort();
        assert_eq!(
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ],
            ret
        );
    }
}
