use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(&root, None)
    }
    pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, minmax: Option<(i32, i32)>) -> i32 {
        let mut ret = if let Some(m) = minmax { m.1 - m.0 } else { 0 };
        if let Some(n) = node {
            let val = n.borrow().val;
            let next = Some(if let Some(m) = minmax {
                (std::cmp::min(m.0, val), std::cmp::max(m.1, val))
            } else {
                (val, val)
            });
            ret = std::cmp::max(ret, Solution::dfs(&n.borrow().left, next));
            ret = std::cmp::max(ret, Solution::dfs(&n.borrow().right, next));
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
            7,
            Solution::max_ancestor_diff(to_tree(vec![
                Some(8),
                Some(3),
                Some(10),
                Some(1),
                Some(6),
                None,
                Some(14),
                None,
                None,
                Some(4),
                Some(7),
                Some(13)
            ]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            3,
            Solution::max_ancestor_diff(to_tree(vec![
                Some(1),
                None,
                Some(2),
                None,
                Some(0),
                Some(3)
            ]))
        );
    }
}
