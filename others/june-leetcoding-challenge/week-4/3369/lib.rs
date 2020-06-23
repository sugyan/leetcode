use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::helper(&root)
    }
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = node {
            let ld = {
                let mut h = 1;
                let mut p = n.borrow().left.clone();
                while let Some(l) = p {
                    h += 1;
                    p = l.borrow().left.clone();
                }
                h
            };
            let rd = {
                let mut h = 1;
                let mut p = n.borrow().right.clone();
                while let Some(r) = p {
                    h += 1;
                    p = r.borrow().right.clone();
                }
                h
            };
            if ld == rd {
                2i32.pow(ld) - 1
            } else {
                1 + Solution::helper(&n.borrow().left) + Solution::helper(&n.borrow().right)
            }
        } else {
            0
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
            6,
            Solution::count_nodes(to_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6)
            ]))
        );
    }
}
