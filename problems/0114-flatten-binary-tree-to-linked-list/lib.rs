use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            Solution::flatten(&mut r.borrow_mut().left);
            let left = r.borrow().left.clone();
            if let Some(l) = left {
                let node = Rc::new(RefCell::new(TreeNode::new(l.borrow().val)));
                node.borrow_mut().left = l.borrow().right.clone();
                node.borrow_mut().right = r.borrow().right.clone();
                r.borrow_mut().right = Some(node);
                r.borrow_mut().left = None;
            }
            Solution::flatten(&mut r.borrow_mut().right);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        let mut tree = to_tree(vec![
            Some(1),
            Some(2),
            Some(5),
            Some(3),
            Some(4),
            None,
            Some(6),
        ]);
        Solution::flatten(&mut tree);
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
            tree
        );
    }
}
