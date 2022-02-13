pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer = Vec::new();
        Self::backtrack(&nums, &mut Vec::new(), &mut answer);
        answer
    }
    fn backtrack(nums: &[i32], v: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>) {
        answer.push(v.clone());
        for (i, &num) in nums.iter().enumerate() {
            v.push(num);
            Self::backtrack(&nums[i + 1..], v, answer);
            v.pop();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::subsets(vec![1, 2, 3]);
        for v in &mut ret {
            v.sort_unstable();
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

    #[test]
    fn example_2() {
        let mut ret = Solution::subsets(vec![0]);
        for v in &mut ret {
            v.sort_unstable();
        }
        ret.sort();
        assert_eq!(vec![vec![], vec![0]], ret);
    }
}
