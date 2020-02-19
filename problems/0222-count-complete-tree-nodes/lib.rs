use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let ld = {
                let mut h = 1;
                let mut node = r.borrow().left.clone();
                while let Some(n) = node {
                    h += 1;
                    node = n.borrow().left.clone();
                }
                h
            };
            let rd = {
                let mut h = 1;
                let mut node = r.borrow().right.clone();
                while let Some(n) = node {
                    h += 1;
                    node = n.borrow().right.clone();
                }
                h
            };
            if ld == rd {
                2i32.pow(ld) - 1
            } else {
                1 + Solution::count_nodes(r.borrow().left.clone())
                    + Solution::count_nodes(r.borrow().right.clone())
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
