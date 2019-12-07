pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        let mut v = Vec::new();
        Solution::helper(&candidates, target, &mut answer, &mut v);
        return answer;
    }
    fn helper(candidates: &Vec<i32>, target: i32, answer: &mut Vec<Vec<i32>>, v: &mut Vec<i32>) {
        if target == 0 {
            answer.push(v.clone());
            return;
        }
        for num in candidates {
            if let Some(last) = v.last() {
                if num < last {
                    continue;
                }
            }
            if target >= *num {
                v.push(*num);
                Solution::helper(candidates, target - num, answer, v);
                v.pop();
            }
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
}
