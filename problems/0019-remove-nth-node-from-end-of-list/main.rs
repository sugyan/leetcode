use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut node = ListNode::new(0);
        node.next = head;
        let dummy = &Some(Box::new(node));
        let (mut n1, mut n2) = (dummy, dummy);
        for _ in 0..n + 1 {
            if let Some(n) = n1 {
                n1 = &n.next;
            }
        }
        while n1.is_some() {
            if let Some(n) = n1 {
                n1 = &n.next;
            }
            if let Some(n) = n2 {
                n2 = &n.next;
            }
        }
        if let Some(n) = n2 {
            // TODO
            // n.next = None;
        }
        return dummy.to_owned().unwrap().next;
    }
}

fn main() {
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            to_list(vec![1, 2, 3, 5]),
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2)
        );
    }
}
