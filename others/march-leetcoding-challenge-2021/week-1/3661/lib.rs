use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut v = Vec::new();
        Self::dfs(&root, 0, &mut v);
        v.iter()
            .map(|v| v.iter().sum::<i64>() as f64 / v.len() as f64)
            .collect()
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, v: &mut Vec<Vec<i64>>) {
        if let Some(n) = node {
            if level >= v.len() {
                v.push(Vec::new());
            }
            v[level].push(i64::from(n.borrow().val));
            Self::dfs(&n.borrow().left, level + 1, v);
            Self::dfs(&n.borrow().right, level + 1, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        let ret = Solution::average_of_levels(to_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]));
        assert!(vec![3.0, 14.5, 11.0]
            .iter()
            .zip(ret.iter())
            .all(|(expected, actual)| (expected - actual).abs() < std::f64::EPSILON));
    }
}
