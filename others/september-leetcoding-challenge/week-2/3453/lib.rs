use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::helper(&root, 0)
    }
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>, num: i32) -> i32 {
        if let Some(n) = node {
            let val = num * 2 + n.borrow().val;
            if n.borrow().left.is_none() && n.borrow().right.is_none() {
                val
            } else {
                Solution::helper(&n.borrow().left, val) + Solution::helper(&n.borrow().right, val)
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
            22,
            Solution::sum_root_to_leaf(to_tree(vec![
                Some(1),
                Some(0),
                Some(1),
                Some(0),
                Some(1),
                Some(0),
                Some(1)
            ]))
        );
    }
}
