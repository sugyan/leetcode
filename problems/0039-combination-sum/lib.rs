pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut answer = Vec::new();
        Self::helper(&candidates, target, &mut Vec::new(), &mut answer);
        answer
    }
    fn helper(candidates: &[i32], target: i32, v: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>) {
        if target == 0 {
            return answer.push(v.clone());
        }
        for &candidate in candidates {
            if v.last().map_or(false, |&last| candidate < last) || candidate > target {
                continue;
            }
            v.push(candidate);
            Self::helper(candidates, target - candidate, v, answer);
            v.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        ret.sort();
        assert_eq!(vec![vec![2, 2, 3], vec![7]], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::combination_sum(vec![2, 3, 5], 8);
        ret.sort();
        assert_eq!(vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]], ret);
    }

    #[test]
    fn example_3() {
        let mut ret = Solution::combination_sum(vec![2], 1);
        ret.sort();
        assert_eq!(Vec::<Vec<i32>>::new(), ret);
    }
}
