use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut answer = 0;
        Solution::dfs(&root, &mut answer);
        answer as i32
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, answer: &mut usize) -> usize {
        if let Some(n) = node {
            let l = Solution::dfs(&n.borrow().left, answer);
            let r = Solution::dfs(&n.borrow().right, answer);
            *answer = std::cmp::max(*answer, l + r);
            std::cmp::max(l, r) + 1
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
}
