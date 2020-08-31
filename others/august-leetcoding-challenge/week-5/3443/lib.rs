use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::helper(&root, key)
    }
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = node {
            let val = n.borrow().val;
            match val.cmp(&key) {
                std::cmp::Ordering::Greater => {
                    let l = Solution::helper(&n.borrow().left, key);
                    n.borrow_mut().left = l;
                }
                std::cmp::Ordering::Less => {
                    let r = Solution::helper(&n.borrow().right, key);
                    n.borrow_mut().right = r;
                }
                std::cmp::Ordering::Equal => {
                    if n.borrow().left.is_none() {
                        return n.borrow().right.clone();
                    }
                    if n.borrow().right.is_none() {
                        return n.borrow().left.clone();
                    }
                    let next = Solution::search_next(&n.borrow().right);
                    if let Some(val) = next {
                        let r = Solution::helper(&n.borrow().right, val);
                        n.borrow_mut().val = val;
                        n.borrow_mut().right = r;
                    }
                }
            }
        }
        node.clone()
    }
    fn search_next(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(n) = node {
            if n.borrow().left.is_some() {
                Solution::search_next(&n.borrow().left)
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
}
