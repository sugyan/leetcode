use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = {
            let mut ret = 0;
            let mut node = &head;
            while let Some(n) = node {
                ret += 1;
                node = &n.next;
            }
            ret
        };
        if len <= 1 {
            return head;
        }
        let mut node = head.as_mut();
        for _ in 0..len / 2 - 1 {
            if let Some(n) = node {
                node = n.next.as_mut();
            }
        }
        let second = node.unwrap().next.take();
        let first = head;
        Self::merge(Self::sort_list(first), Self::sort_list(second))
    }
    fn merge(
        mut first: Option<Box<ListNode>>,
        mut second: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut node = &mut dummy;
        while let (Some(nf), Some(ns)) = (first.as_mut(), second.as_mut()) {
            if nf.val <= ns.val {
                let tmp = nf.next.take();
                node.next = first.take();
                first = tmp;
            } else {
                let tmp = ns.next.take();
                node.next = second.take();
                second = tmp;
            }
            node = node.next.as_mut().unwrap();
        }
        node.next = if first.is_some() {
            first.take()
        } else {
            second.take()
        };
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
            to_list(vec![1, 2, 3, 4]),
            Solution::sort_list(to_list(vec![4, 2, 1, 3]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(vec![-1, 0, 3, 4, 5]),
            Solution::sort_list(to_list(vec![-1, 5, 3, 4, 0]))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(to_list(vec![]), Solution::sort_list(to_list(vec![])));
    }
}
