use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut answer = 0;
        Self::dfs(&root, &mut answer);
        answer
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, answer: &mut i32) -> i32 {
        if let Some(n) = node {
            let l = Self::dfs(&n.borrow().left, answer);
            let r = Self::dfs(&n.borrow().right, answer);
            *answer = (l + r).max(*answer);
            l.max(r) + 1
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
            3,
            Solution::diameter_of_binary_tree(to_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5)
            ]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::diameter_of_binary_tree(to_tree(vec![Some(1), Some(2)]))
        );
    }
}
