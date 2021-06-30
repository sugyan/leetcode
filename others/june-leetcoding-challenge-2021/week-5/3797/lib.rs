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
    fn helper(
        root: &Option<Rc<RefCell<TreeNode>>>,
        p: &Option<Rc<RefCell<TreeNode>>>,
        q: &Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if p == root || q == root {
            return root.clone();
        }
        if let Some(n) = root {
            let l = Self::helper(&n.borrow().left, p, q);
            let r = Self::helper(&n.borrow().right, p, q);
            match (l.is_some(), r.is_some()) {
                (false, _) => r,
                (true, false) => l,
                (true, true) => Some(n.clone()),
            }
        } else {
            root.clone()
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
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(0),
                Some(8),
                None,
                None,
                Some(7),
                Some(4)
            ]),
            Solution::lowest_common_ancestor(
                to_tree(vec![
                    Some(3),
                    Some(5),
                    Some(1),
                    Some(6),
                    Some(2),
                    Some(0),
                    Some(8),
                    None,
                    None,
                    Some(7),
                    Some(4)
                ]),
                to_tree(vec![
                    Some(5),
                    Some(6),
                    Some(2),
                    None,
                    None,
                    Some(7),
                    Some(4)
                ]),
                to_tree(vec![Some(1), Some(0), Some(8)]),
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![
                Some(5),
                Some(6),
                Some(2),
                None,
                None,
                Some(7),
                Some(4)
            ]),
            Solution::lowest_common_ancestor(
                to_tree(vec![
                    Some(3),
                    Some(5),
                    Some(1),
                    Some(6),
                    Some(2),
                    Some(0),
                    Some(8),
                    None,
                    None,
                    Some(7),
                    Some(4)
                ]),
                to_tree(vec![
                    Some(5),
                    Some(6),
                    Some(2),
                    None,
                    None,
                    Some(7),
                    Some(4)
                ]),
                to_tree(vec![Some(4)]),
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_tree(vec![Some(1), Some(2)]),
            Solution::lowest_common_ancestor(
                to_tree(vec![Some(1), Some(2)]),
                to_tree(vec![Some(1), Some(2)]),
                to_tree(vec![Some(2)]),
            )
        );
    }
}
