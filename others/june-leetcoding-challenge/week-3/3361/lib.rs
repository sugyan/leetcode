use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::dfs(&root, val)
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = node {
            let val = n.borrow().val;
            if val == target {
                return node.clone();
            }
            if val > target {
                Solution::dfs(&n.borrow().left, target)
            } else {
                Solution::dfs(&n.borrow().right, target)
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
            to_tree(vec![Some(2), Some(1), Some(3)]),
            Solution::search_bst(
                to_tree(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]),
                2
            )
        );
    }
}
