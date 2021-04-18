use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let len = {
            let mut ret = 0;
            let mut node = &head;
            while let Some(n) = node {
                ret += 1;
                node = &n.next;
            }
            ret
        };
        let mut i = 0;
        let mut head = head;
        let mut dummy = ListNode::new(0);
        let mut p = &mut dummy;
        while let Some(mut h) = head.take() {
            let next = h.next.take();
            if i != len - n {
                p.next = Some(h);
                p = p.next.as_mut().unwrap();
            }
            head = next;
            i += 1;
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
