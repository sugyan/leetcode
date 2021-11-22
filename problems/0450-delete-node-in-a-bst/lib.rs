use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let val = node.borrow().val;
            match val.cmp(&key) {
                Ordering::Greater => {
                    let l = Self::delete_node(node.borrow().left.clone(), key);
                    node.borrow_mut().left = l;
                }
                Ordering::Less => {
                    let r = Self::delete_node(node.borrow().right.clone(), key);
                    node.borrow_mut().right = r;
                }
                Ordering::Equal => {
                    if node.borrow().left.is_none() {
                        return node.borrow().right.clone();
                    }
                    if node.borrow().right.is_none() {
                        return node.borrow().left.clone();
                    }
                    let min = Self::get_min(&node.borrow().right);
                    if let Some(m) = min {
                        let r = Self::delete_node(node.borrow().right.clone(), m);
                        node.borrow_mut().val = m;
                        node.borrow_mut().right = r;
                    }
                }
            }
            Some(node)
        } else {
            root
        }
    }
    fn get_min(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(n) = node {
            if n.borrow().left.is_some() {
                Self::get_min(&n.borrow().left)
            } else {
                Some(n.borrow().val)
            }
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
                Some(5),
                Some(4),
                Some(6),
                Some(2),
                None,
                None,
                Some(7)
            ]),
            Solution::delete_node(
                to_tree(vec![
                    Some(5),
                    Some(3),
                    Some(6),
                    Some(2),
                    Some(4),
                    None,
                    Some(7)
                ]),
                3
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![
                Some(5),
                Some(3),
                Some(6),
                Some(2),
                Some(4),
                None,
                Some(7)
            ]),
            Solution::delete_node(
                to_tree(vec![
                    Some(5),
                    Some(3),
                    Some(6),
                    Some(2),
                    Some(4),
                    None,
                    Some(7)
                ]),
                0
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(None, Solution::delete_node(None, 0));
    }
}
