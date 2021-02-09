use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut greater = 0;
        Self::dfs(&root, &mut greater)
    }
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        greater: &mut i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = node {
            let right = Self::dfs(&n.borrow().right, greater);
            *greater += n.borrow().val;
            let val = *greater;
            let left = Self::dfs(&n.borrow().left, greater);
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
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
                Some(30),
                Some(36),
                Some(21),
                Some(36),
                Some(35),
                Some(26),
                Some(15),
                None,
                None,
                None,
                Some(33),
                None,
                None,
                None,
                Some(8)
            ]),
            Solution::convert_bst(to_tree(vec![
                Some(4),
                Some(1),
                Some(6),
                Some(0),
                Some(2),
                Some(5),
                Some(7),
                None,
                None,
                None,
                Some(3),
                None,
                None,
                None,
                Some(8)
            ]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![Some(1), None, Some(1)]),
            Solution::convert_bst(to_tree(vec![Some(0), None, Some(1)]))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_tree(vec![Some(3), Some(3), Some(2)]),
            Solution::convert_bst(to_tree(vec![Some(1), Some(0), Some(2)]))
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            to_tree(vec![Some(7), Some(9), Some(4), Some(10)]),
            Solution::convert_bst(to_tree(vec![Some(3), Some(2), Some(4), Some(1)]))
        );
    }
}
