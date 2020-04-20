use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::helper(&preorder)
    }
    fn helper(v: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(first) = v.first() {
            let node = Rc::new(RefCell::new(TreeNode::new(*first)));
            let i = (1..v.len())
                .find(|&i| v[i] > v[0])
                .unwrap_or_else(|| v.len());
            node.borrow_mut().left = Solution::helper(&v[1..i]);
            node.borrow_mut().right = Solution::helper(&v[i..]);
            Some(node)
        } else {
            None
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
            to_tree(vec![
                Some(8),
                Some(5),
                Some(10),
                Some(1),
                Some(7),
                None,
                Some(12)
            ]),
            Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12])
        );
    }
}
