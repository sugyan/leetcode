use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::helper(&preorder, &inorder)
    }
    fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(first) = preorder.first() {
            let root = Rc::new(RefCell::new(TreeNode::new(*first)));
            if let Some(idx) = inorder.iter().position(|v| *v == *first) {
                root.borrow_mut().left = Solution::helper(&preorder[1..=idx], &inorder[0..idx]);
                root.borrow_mut().right =
                    Solution::helper(&preorder[idx + 1..], &inorder[idx + 1..]);
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
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
        );
    }
}
