use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut trees = trees;
        trees.sort();
        let mut stack = Vec::<Vec<_>>::new();
        for tree in trees.iter() {
            while stack.len() > 1
                && Self::orientation(&stack[stack.len() - 2], &stack[stack.len() - 1], &tree) > 0
            {
                stack.pop();
            }
            stack.push(tree.clone());
        }
        for tree in trees.iter().rev() {
            while stack.len() > 1
                && Self::orientation(&stack[stack.len() - 2], &stack[stack.len() - 1], &tree) > 0
            {
                stack.pop();
            }
            stack.push(tree.clone());
        }
        stack
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
    fn orientation(p: &[i32], q: &[i32], r: &[i32]) -> i32 {
        (q[1] - p[1]) * (r[0] - q[0]) - (q[0] - p[0]) * (r[1] - q[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::outer_trees(vec![
            vec![1, 1],
            vec![2, 2],
            vec![2, 0],
            vec![2, 4],
            vec![3, 3],
            vec![4, 2],
        ]);
        ret.sort();
        assert_eq!(
            vec![vec![1, 1], vec![2, 0], vec![2, 4], vec![3, 3], vec![4, 2]],
            ret
        );
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::outer_trees(vec![vec![1, 2], vec![2, 2], vec![4, 2]]);
        ret.sort();
        assert_eq!(vec![vec![1, 2], vec![2, 2], vec![4, 2]], ret);
    }
}
