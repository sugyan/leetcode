use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut k = k;
        Solution::inorder(&root, &mut k).unwrap()
    }
    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> Option<i32> {
        if let Some(n) = node {
            if let Some(val) = Solution::inorder(&n.borrow().left, k) {
                return Some(val);
            }
            *k -= 1;
            if *k == 0 {
                return Some(n.borrow().val);
            }
            if let Some(val) = Solution::inorder(&n.borrow().right, k) {
                return Some(val);
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
            1,
            Solution::kth_smallest(to_tree(vec![Some(3), Some(1), Some(4), None, Some(2)]), 1)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            3,
            Solution::kth_smallest(
                to_tree(vec![
                    Some(5),
                    Some(3),
                    Some(6),
                    Some(2),
                    Some(4),
                    None,
                    None,
                    Some(1)
                ]),
                3
            )
        );
    }
}
