use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&root)
    }
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        node.as_ref().map(|n| {
            Rc::new(RefCell::new(TreeNode {
                val: n.borrow().val,
                left: Self::helper(&n.borrow().right),
                right: Self::helper(&n.borrow().left),
            }))
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
            to_tree(vec![
                Some(4),
                Some(7),
                Some(2),
                Some(9),
                Some(6),
                Some(3),
                Some(1)
            ]),
            Solution::invert_tree(to_tree(vec![
                Some(4),
                Some(2),
                Some(7),
                Some(1),
                Some(3),
                Some(6),
                Some(9)
            ]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![Some(2), Some(3), Some(1)]),
            Solution::invert_tree(to_tree(vec![Some(2), Some(1), Some(3)]))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(None, Solution::invert_tree(None));
    }
}
