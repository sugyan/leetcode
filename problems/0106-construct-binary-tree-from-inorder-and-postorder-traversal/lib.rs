use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::helper(&inorder, &postorder)
    }
    fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(last) = postorder.last() {
            let root = Rc::new(RefCell::new(TreeNode::new(*last)));
            if let Some(idx) = inorder.iter().position(|&v| v == *last) {
                root.borrow_mut().left = Solution::helper(&inorder[0..idx], &postorder[0..idx]);
                root.borrow_mut().right =
                    Solution::helper(&inorder[idx + 1..], &postorder[idx..postorder.len() - 1]);
            }
            return Some(root);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        assert_eq!(
            to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ]),
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])
        );
    }
}
