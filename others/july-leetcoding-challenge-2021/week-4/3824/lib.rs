use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            let val = r.borrow().val;
            let left = Self::prune_tree(r.borrow().left.clone());
            let right = Self::prune_tree(r.borrow().right.clone());
            if val != 0 || left.is_some() || right.is_some() {
                return Some(Rc::new(RefCell::new(TreeNode { val, left, right })));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        assert_eq!(
            to_tree(vec![Some(1), None, Some(0), None, Some(1)]),
            Solution::prune_tree(to_tree(vec![Some(1), None, Some(0), Some(0), Some(1)]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![Some(1), None, Some(1), None, Some(1)]),
            Solution::prune_tree(to_tree(vec![
                Some(1),
                Some(0),
                Some(1),
                Some(0),
                Some(0),
                Some(0),
                Some(1)
            ]))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_tree(vec![
                Some(1),
                Some(1),
                Some(0),
                Some(1),
                Some(1),
                None,
                Some(1)
            ]),
            Solution::prune_tree(to_tree(vec![
                Some(1),
                Some(1),
                Some(0),
                Some(1),
                Some(1),
                Some(0),
                Some(1),
                Some(0)
            ]))
        );
    }
}
