use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(&root, 0)
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, prev: i32) -> i32 {
        if let Some(n) = node {
            let val = prev * 10 + n.borrow().val;
            if n.borrow().left.is_none() && n.borrow().right.is_none() {
                val
            } else {
                Solution::dfs(&n.borrow().left, val) + Solution::dfs(&n.borrow().right, val)
            }
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
            25,
            Solution::sum_numbers(to_tree(vec![Some(1), Some(2), Some(3)]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1026,
            Solution::sum_numbers(to_tree(vec![Some(4), Some(9), Some(0), Some(5), Some(1)]))
        );
    }
}
