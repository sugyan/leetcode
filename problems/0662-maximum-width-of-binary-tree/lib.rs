use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, 0, 0, &mut Vec::new()) as i32
    }
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
        index: usize,
        v: &mut Vec<usize>,
    ) -> usize {
        if let Some(n) = node {
            if depth == v.len() {
                v.push(index);
            }
            let l = Self::dfs(&n.borrow().left, depth + 1, index * 2, v);
            let r = Self::dfs(&n.borrow().right, depth + 1, index * 2 + 1, v);
            (index - v[depth] + 1).max(l.max(r))
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        assert_eq!(
            4,
            Solution::width_of_binary_tree(to_tree(vec![
                Some(1),
                Some(3),
                Some(2),
                Some(5),
                Some(3),
                None,
                Some(9)
            ]))
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            2,
            Solution::width_of_binary_tree(to_tree(vec![Some(1), Some(3), None, Some(5), Some(3)]))
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            2,
            Solution::width_of_binary_tree(to_tree(vec![Some(1), Some(3), Some(2), Some(5)]))
        )
    }
}
