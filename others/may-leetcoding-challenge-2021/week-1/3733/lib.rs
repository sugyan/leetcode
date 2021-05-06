use std::cell::RefCell;
use std::rc::Rc;
use utils::{ListNode, TreeNode};

pub struct Solution;

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut len = 0;
        let mut node = &head;
        while let Some(n) = node {
            len += 1;
            node = &n.next;
        }
        Solution::helper(head, len)
    }
    fn helper(mut head: Option<Box<ListNode>>, len: usize) -> Option<Rc<RefCell<TreeNode>>> {
        let second = {
            let mut node = &mut head;
            for _ in 0..len / 2 {
                if let Some(n) = node {
                    node = &mut n.next;
                }
            }
            node.take()
        };
        if let Some(mut s) = second {
            let right = Self::helper(s.next.take(), len - len / 2 - 1);
            let tree_node = TreeNode {
                val: s.val,
                left: Self::helper(head, len / 2),
                right,
            };
            Some(Rc::new(RefCell::new(tree_node)))
        } else {
            None
        }
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

    #[test]
    fn example_2() {
        assert_eq!(None, Solution::sorted_list_to_bst(to_list(Vec::new())));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_tree(vec![Some(0)]),
            Solution::sorted_list_to_bst(to_list(vec![0]))
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            to_tree(vec![Some(3), Some(1)]),
            Solution::sorted_list_to_bst(to_list(vec![1, 3]))
        );
    }
}
