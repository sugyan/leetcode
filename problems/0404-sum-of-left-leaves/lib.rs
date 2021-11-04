use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, false)
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        if let Some(n) = node {
            let n = n.borrow();
            if n.left.is_none() && n.right.is_none() && is_left {
                n.val
            } else {
                Self::dfs(&n.left, true) + Self::dfs(&n.right, false)
            }
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
            24,
            Solution::sum_of_left_leaves(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::sum_of_left_leaves(to_tree(vec![Some(1)])));
    }
}
