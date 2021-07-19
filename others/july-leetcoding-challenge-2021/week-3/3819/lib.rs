use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&root, &p, &q)
    }
    pub fn helper(
        root: &Option<Rc<RefCell<TreeNode>>>,
        p: &Option<Rc<RefCell<TreeNode>>>,
        q: &Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root == p || root == q {
            return root.clone();
        }
        if let Some(node) = root {
            let l = Self::helper(&node.borrow().left, p, q);
            let r = Self::helper(&node.borrow().right, p, q);
            if l.is_some() && r.is_some() {
                return root.clone();
            }
            if l.is_none() {
                r
            } else {
                l
            }
        } else {
            None
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
            to_tree(vec![
                Some(6),
                Some(2),
                Some(8),
                Some(0),
                Some(4),
                Some(7),
                Some(9),
                None,
                None,
                Some(3),
                Some(5)
            ]),
            Solution::lowest_common_ancestor(
                to_tree(vec![
                    Some(6),
                    Some(2),
                    Some(8),
                    Some(0),
                    Some(4),
                    Some(7),
                    Some(9),
                    None,
                    None,
                    Some(3),
                    Some(5)
                ]),
                to_tree(vec![
                    Some(2),
                    Some(0),
                    Some(4),
                    None,
                    None,
                    Some(3),
                    Some(5)
                ]),
                to_tree(vec![Some(8), Some(7), Some(9)])
            )
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![
                Some(2),
                Some(0),
                Some(4),
                None,
                None,
                Some(3),
                Some(5)
            ]),
            Solution::lowest_common_ancestor(
                to_tree(vec![
                    Some(6),
                    Some(2),
                    Some(8),
                    Some(0),
                    Some(4),
                    Some(7),
                    Some(9),
                    None,
                    None,
                    Some(3),
                    Some(5)
                ]),
                to_tree(vec![
                    Some(2),
                    Some(0),
                    Some(4),
                    None,
                    None,
                    Some(3),
                    Some(5)
                ]),
                to_tree(vec![Some(4), Some(3), Some(5)])
            )
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_tree(vec![Some(2), Some(1)]),
            Solution::lowest_common_ancestor(
                to_tree(vec![Some(2), Some(1)]),
                to_tree(vec![Some(2), Some(1)]),
                to_tree(vec![Some(1)])
            )
        )
    }
}
