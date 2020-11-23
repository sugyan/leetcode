use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let ret = Solution::dfs(&root);
        std::cmp::max(ret.0, ret.1)
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(n) = node {
            let l = Solution::dfs(&n.borrow().left);
            let r = Solution::dfs(&n.borrow().right);
            (
                n.borrow().val + l.1 + r.1,
                std::cmp::max(l.0, l.1) + std::cmp::max(r.0, r.1),
            )
        } else {
            (0, 0)
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
            7,
            Solution::rob(to_tree(vec![
                Some(3),
                Some(2),
                Some(3),
                None,
                Some(3),
                None,
                Some(1)
            ]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            9,
            Solution::rob(to_tree(vec![
                Some(3),
                Some(4),
                Some(5),
                Some(1),
                Some(3),
                None,
                Some(1)
            ]))
        );
    }
}
