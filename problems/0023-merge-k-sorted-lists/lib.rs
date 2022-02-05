use std::cmp::Reverse;
use std::collections::BinaryHeap;
use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let mut bh = BinaryHeap::new();
        for (i, list) in lists.iter_mut().enumerate() {
            if let Some(node) = list {
                bh.push((Reverse(node.val), i));
                *list = node.next.take();
            }
        }
        let mut dummy = ListNode::new(0);
        let mut p = &mut dummy;
        while let Some((Reverse(min), i)) = bh.pop() {
            p.next = Some(Box::new(ListNode::new(min)));
            p = p.next.as_mut().unwrap();
            if let Some(node) = lists[i].take() {
                bh.push((Reverse(node.val), i));
                lists[i] = node.next;
            }
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6]),
            Solution::merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6])
            ])
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(None, Solution::merge_k_lists(vec![]))
    }

    #[test]
    fn example_3() {
        assert_eq!(None, Solution::merge_k_lists(vec![None]))
    }
}
