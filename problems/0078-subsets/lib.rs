pub struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer = Vec::new();
        answer.push(vec![]);
        for num in nums {
            for i in 0..answer.len() {
                let mut v = answer[i].clone();
                v.push(num);
                answer.push(v);
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
        let mut ret = Solution::subsets(vec![1, 2, 3]);
        for v in ret.iter_mut() {
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
}
