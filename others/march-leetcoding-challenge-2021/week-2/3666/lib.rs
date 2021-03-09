use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        v: i32,
        d: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if d == 1 {
            Some(Rc::new(RefCell::new(TreeNode {
                val: v,
                left: root,
                right: None,
            })))
        } else {
            Self::dfs(&root, v, d - 2);
            root
        }
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, v: i32, d: i32) {
        if let Some(n) = node {
            if d > 0 {
                Self::dfs(&n.borrow().left, v, d - 1);
                Self::dfs(&n.borrow().right, v, d - 1);
            } else {
                let mut n = n.borrow_mut();
                n.left = Some(Rc::new(RefCell::new(TreeNode {
                    val: v,
                    left: n.left.take(),
                    right: None,
                })));
                n.right = Some(Rc::new(RefCell::new(TreeNode {
                    val: v,
                    left: None,
                    right: n.right.take(),
                })));
            }
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
                Some(4),
                Some(1),
                Some(1),
                Some(2),
                None,
                None,
                Some(6),
                Some(3),
                Some(1),
                Some(5)
            ]),
            Solution::add_one_row(
                to_tree(vec![Some(4), Some(2), Some(6), Some(3), Some(1), Some(5)]),
                1,
                2
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![
                Some(4),
                Some(2),
                None,
                Some(1),
                Some(1),
                Some(3),
                None,
                None,
                Some(1),
            ]),
            Solution::add_one_row(
                to_tree(vec![Some(4), Some(2), None, Some(3), Some(1)]),
                1,
                3
            )
        );
    }
}
