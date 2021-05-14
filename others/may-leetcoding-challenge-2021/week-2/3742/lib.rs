use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut bm = node.borrow_mut();
            Self::flatten(&mut bm.left);
            if let Some(left) = bm.left.take() {
                let val = left.borrow().val;
                bm.right = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: left.borrow_mut().right.take(),
                    right: bm.right.take(),
                })));
            }
            Self::flatten(&mut bm.right);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        let mut root = to_tree(vec![
            Some(1),
            Some(2),
            Some(5),
            Some(3),
            Some(4),
            None,
            Some(6),
        ]);
        Solution::flatten(&mut root);
        assert_eq!(
            to_tree(vec![
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
                None,
                Some(4),
                None,
                Some(5),
                None,
                Some(6)
            ]),
            root
        );
    }

    #[test]
    fn example_2() {
        let mut root = None;
        Solution::flatten(&mut root);
        assert_eq!(None, root);
    }

    #[test]
    fn example_3() {
        let mut root = to_tree(vec![Some(0)]);
        Solution::flatten(&mut root);
        assert_eq!(to_tree(vec![Some(0)]), root);
    }
}
