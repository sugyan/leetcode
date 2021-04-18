use std::cmp::Ordering;
use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut n = n;
        let mut dummy = ListNode::new(0);
        let mut p1 = &head.clone();
        let mut p2 = &mut dummy;
        while let Some(p) = p1 {
            p1 = &p.next;
            match n.cmp(&0) {
                Ordering::Less => {}
                Ordering::Equal => {
                    if let Some(mut h) = head.take() {
                        let next = h.next.take();
                        p2.next = Some(h);
                        p2 = p2.next.as_mut().unwrap();
                        head = next;
                    }
                }
                Ordering::Greater => n -= 1,
            }
        }
        if let Some(h) = head {
            p2.next = h.next;
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
            to_list(vec![1, 2, 3, 5]),
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(Vec::new()),
            Solution::remove_nth_from_end(to_list(vec![1]), 1)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_list(vec![1]),
            Solution::remove_nth_from_end(to_list(vec![1, 2]), 1)
        );
    }
}
