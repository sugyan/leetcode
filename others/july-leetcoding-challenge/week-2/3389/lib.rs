use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Solution::helper(&p, &q)
    }
    fn helper(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                p.borrow().val == q.borrow().val
                    && Solution::helper(&p.borrow().left, &q.borrow().left)
                    && Solution::helper(&p.borrow().right, &q.borrow().right)
            }
            (None, None) => true,
            _ => false,
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
            true,
            Solution::is_same_tree(
                to_tree(vec![Some(1), Some(2), Some(3)]),
                to_tree(vec![Some(1), Some(2), Some(3)]),
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::is_same_tree(
                to_tree(vec![Some(1), Some(2)]),
                to_tree(vec![Some(1), None, Some(2)]),
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::is_same_tree(
                to_tree(vec![Some(1), Some(2), Some(1)]),
                to_tree(vec![Some(1), Some(1), Some(2)]),
            )
        );
    }
}
