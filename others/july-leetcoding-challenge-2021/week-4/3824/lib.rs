use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if Self::dfs(&root) == 0 {
            return None;
        }
        if let Some(r) = root {
            Some(Rc::new(RefCell::new(TreeNode {
                val: r.borrow().val,
                left: Self::prune_tree(r.borrow().left.clone()),
                right: Self::prune_tree(r.borrow().right.clone()),
            })))
        } else {
            None
        }
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ret = 0;
        if let Some(n) = node {
            ret += n.borrow().val;
            ret += Self::dfs(&n.borrow().left);
            ret += Self::dfs(&n.borrow().right);
        }
        ret
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
