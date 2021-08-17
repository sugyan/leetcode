use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(&root, None)
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max: Option<i32>) -> i32 {
        if let Some(n) = node {
            let val = n.borrow().val;
            let m = Some(max.map_or(val, |m| m.max(val)));
            (if m == Some(val) { 1 } else { 0 })
                + Solution::dfs(&n.borrow().left, m)
                + Solution::dfs(&n.borrow().right, m)
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
            4,
            Solution::good_nodes(to_tree(vec![
                Some(3),
                Some(1),
                Some(4),
                Some(3),
                None,
                Some(1),
                Some(5)
            ]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            3,
            Solution::good_nodes(to_tree(vec![Some(3), Some(3), None, Some(4), Some(2)]))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::good_nodes(to_tree(vec![Some(1)])));
    }
}
