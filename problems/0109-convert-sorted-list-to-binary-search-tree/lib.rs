use std::cell::RefCell;
use std::rc::Rc;
use utils::{ListNode, TreeNode};

pub struct Solution {}

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = &head;
        let mut size = 0;
        while let Some(n) = node {
            size += 1;
            node = &n.next;
        }
        Solution::helper(&mut &head, 0, size - 1)
    }
    fn helper(list: &mut &Option<Box<ListNode>>, l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if l > r {
            return None;
        }
        let m = (l + r) / 2;
        let left = Solution::helper(list, l, m - 1);
        if let Some(n) = list {
            let root = Rc::new(RefCell::new(TreeNode::new(n.val)));
            root.borrow_mut().left = left;
            *list = &n.next;
            root.borrow_mut().right = Solution::helper(list, m + 1, r);
            return Some(root);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::{to_list, to_tree};

    #[test]
    fn example_1() {
        assert_eq!(
            to_tree(vec![
                Some(0),
                Some(-10),
                Some(5),
                None,
                Some(-3),
                None,
                Some(9)
            ]),
            Solution::sorted_list_to_bst(to_list(vec![-10, -3, 0, 5, 9]))
        );
    }
}
