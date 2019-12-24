use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy1 = Some(Box::new(ListNode::new(0)));
        let mut dummy2 = Some(Box::new(ListNode::new(0)));
        let mut node1 = &mut dummy1;
        let mut node2 = &mut dummy2;
        let mut list = head;
        let mut node = &mut list;
        while let Some(n) = node {
            if n.val < x {
                if let Some(n1) = node1 {
                    n1.next = Some(Box::new(ListNode::new(n.val)));
                    node1 = &mut n1.next;
                }
            } else {
                if let Some(n2) = node2 {
                    n2.next = Some(Box::new(ListNode::new(n.val)));
                    node2 = &mut n2.next;
                }
            }
            node = &mut n.next;
        }
        if let Some(n1) = node1 {
            n1.next = dummy2.unwrap().next
        }
        return dummy1.unwrap().next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            to_list(vec![1, 2, 2, 4, 3, 5]),
            Solution::partition(to_list(vec![1, 4, 3, 2, 5, 2]), 3)
        );
    }
}
