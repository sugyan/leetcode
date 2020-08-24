use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(&root, false)
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        if let Some(n) = node {
            let (l, r) = (&n.borrow().left, &n.borrow().right);
            if l.is_none() && r.is_none() && is_left {
                return n.borrow().val;
            }
            Solution::dfs(l, true) + Solution::dfs(r, false)
        } else {
            0
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
            24,
            Solution::sum_of_left_leaves(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ]))
        );
    }
}
