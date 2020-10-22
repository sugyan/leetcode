use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(&root)
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = node {
            1 + if n.borrow().left.is_none() || n.borrow().right.is_none() {
                Solution::dfs(&n.borrow().left) + Solution::dfs(&n.borrow().right)
            } else {
                std::cmp::min(
                    Solution::dfs(&n.borrow().left),
                    Solution::dfs(&n.borrow().right),
                )
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
            2,
            Solution::min_depth(to_tree(vec![
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
        assert_eq!(
            5,
            Solution::min_depth(to_tree(vec![
                Some(2),
                None,
                Some(3),
                None,
                Some(4),
                None,
                Some(5),
                None,
                Some(6)
            ]))
        );
    }
}
