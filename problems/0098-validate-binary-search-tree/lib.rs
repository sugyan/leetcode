use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut prev: Option<i32> = None;
        return Solution::inorder(root, &mut prev);
    }
    fn inorder(node: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>) -> bool {
        if let Some(node) = node {
            if let Some(l) = node.borrow().left.as_ref() {
                if !Solution::inorder(Some(l.clone()), prev) {
                    return false;
                }
            }
            if let Some(p) = prev {
                if *p >= node.borrow().val {
                    return false;
                }
            }
            *prev = Some(node.borrow().val);
            if let Some(r) = node.borrow().right.as_ref() {
                if !Solution::inorder(Some(r.clone()), prev) {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::is_valid_bst(to_tree(vec![Some(2), Some(1), Some(3)]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::is_valid_bst(to_tree(vec![
                Some(5),
                Some(1),
                Some(4),
                None,
                None,
                Some(3),
                Some(6)
            ]))
        );
    }
}
