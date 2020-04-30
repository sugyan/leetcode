use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn is_valid_sequence(root: Option<Rc<RefCell<TreeNode>>>, arr: Vec<i32>) -> bool {
        Solution::helper(&root, &arr)
    }

    fn helper(node: &Option<Rc<RefCell<TreeNode>>>, target: &[i32]) -> bool {
        if let Some(n) = node {
            if target.is_empty() || n.borrow().val != target[0] {
                return false;
            }
            let l = &n.borrow().left;
            let r = &n.borrow().right;
            if target.len() == 1 && l.is_none() && r.is_none() {
                return true;
            }
            Solution::helper(l, &target[1..]) || Solution::helper(r, &target[1..])
        } else {
            false
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
            Solution::is_valid_sequence(
                to_tree(vec![
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    None,
                    None,
                    Some(1),
                    Some(0),
                    Some(0)
                ]),
                vec![0, 1, 0, 1]
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::is_valid_sequence(
                to_tree(vec![
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    None,
                    None,
                    Some(1),
                    Some(0),
                    Some(0)
                ]),
                vec![0, 0, 1]
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::is_valid_sequence(
                to_tree(vec![
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    None,
                    None,
                    Some(1),
                    Some(0),
                    Some(0)
                ]),
                vec![0, 1, 1]
            )
        );
    }
}
