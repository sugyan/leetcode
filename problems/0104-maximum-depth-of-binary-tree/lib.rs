use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(&root)
    }
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        node.as_ref().map_or(0, |n| {
            Self::helper(&n.borrow().left).max(Self::helper(&n.borrow().right)) + 1
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::max_depth(to_tree(vec![
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
            2,
            Solution::max_depth(to_tree(vec![Some(1), None, Some(2)]))
        );
    }
}
