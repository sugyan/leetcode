use std::cmp::Reverse;
use std::collections::BinaryHeap;
use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut nodes = lists.iter().collect::<Vec<_>>();
        let mut bh = BinaryHeap::new();
        for (i, node) in nodes.iter_mut().enumerate() {
            if let Some(n) = node {
                bh.push((Reverse(n.val), i));
                *node = &n.next;
            }
        }
        let mut dummy = ListNode::new(0);
        let mut p = &mut dummy;
        while let Some((minval, i)) = bh.pop() {
            p.next = Some(Box::new(ListNode::new(minval.0)));
            p = p.next.as_mut().unwrap();
            if let Some(n) = nodes[i] {
                bh.push((Reverse(n.val), i));
                nodes[i] = &n.next;
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
        assert_eq!(to_list(vec![]), Solution::merge_k_lists(vec![]))
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_list(vec![]),
            Solution::merge_k_lists(vec![to_list(vec![])])
        )
    }
}
