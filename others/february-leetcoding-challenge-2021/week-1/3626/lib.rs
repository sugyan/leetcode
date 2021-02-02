use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            let val = node.borrow().val;
            if val < low {
                return Self::trim_bst(node.borrow().right.clone(), low, high);
            }
            if val > high {
                return Self::trim_bst(node.borrow().left.clone(), low, high);
            }
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: Self::trim_bst(node.borrow().left.clone(), low, high),
                right: Self::trim_bst(node.borrow().right.clone(), low, high),
            })))
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
            to_tree(vec![Some(1), None, Some(2)]),
            Solution::trim_bst(to_tree(vec![Some(1), Some(0), Some(2)]), 1, 2)
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![Some(3), Some(2), None, Some(1)]),
            Solution::trim_bst(
                to_tree(vec![
                    Some(3),
                    Some(0),
                    Some(4),
                    None,
                    Some(2),
                    None,
                    None,
                    Some(1)
                ]),
                1,
                3
            )
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_tree(vec![Some(1)]),
            Solution::trim_bst(to_tree(vec![Some(1)]), 1, 2)
        )
    }

    #[test]
    fn example_4() {
        assert_eq!(
            to_tree(vec![Some(1), None, Some(2)]),
            Solution::trim_bst(to_tree(vec![Some(1), None, Some(2)]), 1, 3)
        )
    }

    #[test]
    fn example_5() {
        assert_eq!(
            to_tree(vec![Some(2)]),
            Solution::trim_bst(to_tree(vec![Some(1), None, Some(2)]), 2, 4)
        )
    }
}
