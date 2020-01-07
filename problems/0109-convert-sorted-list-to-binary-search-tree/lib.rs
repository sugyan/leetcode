use std::cell::RefCell;
use std::rc::Rc;
use utils::{ListNode, TreeNode};

pub struct Solution {}

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut head = head;
        let mut node = &mut head;
        let mut v: Vec<i32> = Vec::new();
        while let Some(n) = node {
            v.push(n.val);
            node = &mut n.next;
        }
        Solution::make_bst(&v)
    }
    fn make_bst(v: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if v.is_empty() {
            return None;
        }
        let m = v.len() / 2;
        let node = Rc::new(RefCell::new(TreeNode::new(v[m])));
        node.borrow_mut().left = Solution::make_bst(&v[0..m]);
        node.borrow_mut().right = Solution::make_bst(&v[m + 1..]);
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::{to_list, to_tree};

    #[test]
    fn example_1() {
        assert_eq!(
            to_tree(vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)]),
            Solution::sorted_list_to_bst(to_list(vec![-10, -3, 0, 5, 9]))
        );
    }
}
