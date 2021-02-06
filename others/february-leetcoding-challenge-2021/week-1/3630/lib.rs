use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer = Vec::new();
        Self::dfs(&root, &mut answer, 0);
        answer
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, answer: &mut Vec<i32>, depth: usize) {
        if let Some(n) = node {
            if depth >= answer.len() {
                answer.push(n.borrow().val);
            } else {
                answer[depth] = n.borrow().val;
            }
            Self::dfs(&n.borrow().left, answer, depth + 1);
            Self::dfs(&n.borrow().right, answer, depth + 1);
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
            vec![1, 3, 4],
            Solution::right_side_view(to_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                None,
                Some(5),
                None,
                Some(4)
            ]))
        )
    }
}
