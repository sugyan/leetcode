use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&preorder, &inorder)
    }
    fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(first) = preorder.first() {
            if let Some(i) = inorder.iter().position(|val| val == first) {
                return Some(Rc::new(RefCell::new(TreeNode {
                    val: *first,
                    left: Self::helper(&preorder[1..=i], &inorder[..i]),
                    right: Self::helper(&preorder[i + 1..], &inorder[i + 1..]),
                })));
            }
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
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![Some(-1),]),
            Solution::build_tree(vec![-1], vec![-1])
        )
    }
}
