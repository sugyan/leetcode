use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::helper(&root)
    }
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = node {
            1 + match (n.borrow().left.is_some(), n.borrow().right.is_some()) {
                (true, true) => std::cmp::min(
                    Solution::helper(&n.borrow().left),
                    Solution::helper(&n.borrow().right),
                ),
                (true, false) => Solution::helper(&n.borrow().left),
                (false, true) => Solution::helper(&n.borrow().right),
                (false, false) => 0,
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
