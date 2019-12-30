use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();
        Solution::dfs(&mut answer, root);
        return answer;
    }
    fn dfs(answer: &mut Vec<i32>, node: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            if node.borrow().left.is_some() {
                Solution::dfs(answer, Some(node.borrow().left.as_ref().unwrap().clone()));
            }
            answer.push(node.borrow().val);
            if node.borrow().right.is_some() {
                Solution::dfs(answer, Some(node.borrow().right.as_ref().unwrap().clone()));
            }
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
            vec![1, 3, 2],
            Solution::inorder_traversal(to_tree(vec![Some(1), None, Some(2), Some(3)]))
        );
    }
}
