use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&inorder, &postorder)
    }
    fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(&last) = postorder.last() {
            if let Some(pos) = inorder.iter().position(|&x| x == last) {
                return Some(Rc::new(RefCell::new(TreeNode {
                    val: last,
                    left: Self::helper(&inorder[0..pos], &postorder[0..pos]),
                    right: Self::helper(&inorder[pos + 1..], &postorder[pos..postorder.len() - 1]),
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
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![Some(-1)]),
            Solution::build_tree(vec![-1], vec![-1])
        );
    }
}
