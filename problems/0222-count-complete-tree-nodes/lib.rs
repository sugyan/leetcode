use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(&root)
    }
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = node {
            let ld = Self::dfs_l(&node);
            let rd = Self::dfs_r(&node);
            if ld == rd {
                2_i32.pow(ld) - 1
            } else {
                1 + Self::helper(&n.borrow().left) + Self::helper(&n.borrow().right)
            }
        } else {
            0
        }
    }
    fn dfs_l(node: &Option<Rc<RefCell<TreeNode>>>) -> u32 {
        if let Some(n) = node {
            Self::dfs_l(&n.borrow().left) + 1
        } else {
            0
        }
    }
    fn dfs_r(node: &Option<Rc<RefCell<TreeNode>>>) -> u32 {
        if let Some(n) = node {
            Self::dfs_r(&n.borrow().right) + 1
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
            6,
            Solution::count_nodes(to_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6)
            ]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::count_nodes(None));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::count_nodes(to_tree(vec![Some(1)])));
    }
}
