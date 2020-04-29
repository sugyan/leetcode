use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut answer = std::i32::MIN;
        Solution::dfs(&root, &mut answer);
        answer
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, answer: &mut i32) -> i32 {
        if let Some(n) = node {
            let val = n.borrow().val;
            let l = std::cmp::max(0, Solution::dfs(&n.borrow().left, answer));
            let r = std::cmp::max(0, Solution::dfs(&n.borrow().right, answer));
            *answer = std::cmp::max(*answer, val + l + r);
            val + std::cmp::max(l, r)
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
            6,
            Solution::max_path_sum(to_tree(vec![Some(1), Some(2), Some(3)]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            42,
            Solution::max_path_sum(to_tree(vec![
                Some(-10),
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
