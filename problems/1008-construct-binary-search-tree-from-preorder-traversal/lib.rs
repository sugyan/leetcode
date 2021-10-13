use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&preorder)
    }
    fn helper(v: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(&first) = v.first() {
            let i = v.iter().position(|&e| e > first).unwrap_or(v.len());
            Some(Rc::new(RefCell::new(TreeNode {
                val: first,
                left: Self::helper(&v[1..i]),
                right: Self::helper(&v[i..]),
            })))
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
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![Some(1), None, Some(3)]),
            Solution::bst_from_preorder(vec![1, 3])
        )
    }
}
