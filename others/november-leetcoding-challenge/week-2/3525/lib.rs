use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = &root {
            Solution::dfs(&root, r.borrow().val, r.borrow().val)
        } else {
            0
        }
    }
    pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32) -> i32 {
        if let Some(n) = node {
            let val = n.borrow().val;
            let min = std::cmp::min(min, val);
            let max = std::cmp::max(max, val);
            std::cmp::max(
                Solution::dfs(&n.borrow().left, min, max),
                Solution::dfs(&n.borrow().right, min, max),
            )
        } else {
            max - min
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
            7,
            Solution::max_ancestor_diff(to_tree(vec![
                Some(8),
                Some(3),
                Some(10),
                Some(1),
                Some(6),
                None,
                Some(14),
                None,
                None,
                Some(4),
                Some(7),
                Some(13)
            ]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            3,
            Solution::max_ancestor_diff(to_tree(vec![
                Some(1),
                None,
                Some(2),
                None,
                Some(0),
                Some(3)
            ]))
        );
    }
}
