pub struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        Solution::helper(&nums, &mut vec![], &mut answer);
        answer
    }
    fn helper(nums: &[i32], v: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>) {
        answer.push(v.clone());
        for (i, &num) in nums.iter().enumerate() {
            v.push(num);
            Solution::helper(&nums[i + 1..], v, answer);
            v.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret: Vec<Vec<i32>> = Solution::subsets(vec![1, 2, 3]);
        for v in ret.iter_mut() {
            v.sort();
        }
        ret.sort();
        assert_eq!(
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3]
            ],
            ret
        );
    }
}
