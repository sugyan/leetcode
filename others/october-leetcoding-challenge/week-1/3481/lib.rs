pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut answer = Vec::new();
        let mut v: Vec<i32> = Vec::new();
        Solution::helper(&candidates, target, &mut v, &mut answer);
        answer
    }
    fn helper(candidates: &[i32], target: i32, v: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>) {
        if target == 0 {
            answer.push(v.clone());
            return;
        }
        for &candidate in candidates.iter() {
            if let Some(&last) = v.last() {
                if candidate < last {
                    continue;
                }
            }
            if candidate <= target {
                v.push(candidate);
                Solution::helper(candidates, target - candidate, v, answer);
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
        for v in ret.iter_mut() {
            v.sort_unstable();
        }
        ret.sort();
        assert_eq!(vec![vec![2, 2, 3], vec![7]], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::combination_sum(vec![2, 3, 5], 8);
        for v in ret.iter_mut() {
            v.sort_unstable();
        }
        ret.sort();
        assert_eq!(vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]], ret);
    }

    #[test]
    fn example_3() {
        let mut ret = Solution::combination_sum(vec![2], 1);
        for v in ret.iter_mut() {
            v.sort_unstable();
        }
        ret.sort();
        let v: Vec<Vec<i32>> = Vec::new();
        assert_eq!(v, ret);
    }

    #[test]
    fn example_4() {
        let mut ret = Solution::combination_sum(vec![1], 1);
        for v in ret.iter_mut() {
            v.sort_unstable();
        }
        ret.sort();
        assert_eq!(vec![vec![1]], ret);
    }

    #[test]
    fn example_5() {
        let mut ret = Solution::combination_sum(vec![1], 2);
        for v in ret.iter_mut() {
            v.sort_unstable();
        }
        ret.sort();
        assert_eq!(vec![vec![1, 1]], ret);
    }
}
