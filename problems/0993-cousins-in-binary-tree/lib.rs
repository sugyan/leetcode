use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut vd = VecDeque::new();
        if let Some(r) = root {
            vd.push_back((r, None));
        }
        while !vd.is_empty() {
            let mut x_found = None;
            let mut y_found = None;
            for _ in 0..vd.len() {
                if let Some((node, parent)) = vd.pop_front() {
                    let val = node.borrow().val;
                    if val == x {
                        x_found = parent;
                    }
                    if val == y {
                        y_found = parent;
                    }
                    if let Some(n) = node.borrow_mut().left.take() {
                        vd.push_back((n, Some(val)));
                    }
                    if let Some(n) = node.borrow_mut().right.take() {
                        vd.push_back((n, Some(val)));
                    }
                }
            }
            if let (Some(x_parent), Some(y_parent)) = (x_found, y_found) {
                return x_parent != y_parent;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        assert_eq!(
            false,
            Solution::is_cousins(to_tree(vec![Some(1), Some(2), Some(3), Some(4)]), 4, 3)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            true,
            Solution::is_cousins(
                to_tree(vec![
                    Some(1),
                    Some(2),
                    Some(3),
                    None,
                    Some(4),
                    None,
                    Some(5)
                ]),
                5,
                4
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::is_cousins(to_tree(vec![Some(1), Some(2), Some(3), Some(4)]), 2, 3)
        );
    }
}
