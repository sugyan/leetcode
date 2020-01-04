use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut node: Option<Rc<RefCell<TreeNode>>> = root;
        while node.is_some() || !stack.is_empty() {
            while let Some(n) = node {
                stack.push(n.clone());
                node = n.borrow().left.clone();
            }
            node = stack.pop();
            if let Some(n) = node {
                answer.push(n.borrow().val);
                node = n.borrow().right.clone();
            }
        }
        answer
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
