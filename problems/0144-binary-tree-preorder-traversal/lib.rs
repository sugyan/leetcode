use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];
        while let Some(last) = stack.pop() {
            if let Some(n) = last {
                answer.push(n.borrow().val);
                stack.push(n.borrow().right.clone());
                stack.push(n.borrow().left.clone());
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
            vec![1, 2, 3],
            Solution::preorder_traversal(to_tree(vec![Some(1), None, Some(2), Some(3)]))
        );
    }
}
