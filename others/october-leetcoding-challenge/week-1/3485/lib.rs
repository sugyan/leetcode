use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::helper(&root, val)
    }
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = node {
            if n.borrow().val < val {
                let node = Solution::helper(&n.borrow().right, val);
                n.borrow_mut().right = node
            } else {
                let node = Solution::helper(&n.borrow().left, val);
                n.borrow_mut().left = node;
            }
        } else {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        node.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        assert_eq!(
            to_tree(vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)]),
            Solution::insert_into_bst(
                to_tree(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]),
                5
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![
                Some(40),
                Some(20),
                Some(60),
                Some(10),
                Some(30),
                Some(50),
                Some(70),
                None,
                None,
                Some(25)
            ]),
            Solution::insert_into_bst(
                to_tree(vec![
                    Some(40),
                    Some(20),
                    Some(60),
                    Some(10),
                    Some(30),
                    Some(50),
                    Some(70)
                ]),
                25
            )
        );
    }
}
