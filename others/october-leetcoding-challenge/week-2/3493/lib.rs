use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::sort(head)
    }
    fn sort(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = Solution::length(&head);
        if len <= 1 {
            return head;
        }
        let mut node = head.as_mut();
        for _ in 0..len / 2 - 1 {
            if let Some(n) = node {
                node = n.next.as_mut();
            }
        }
        let r = node.unwrap().next.take();
        let l = head;
        Solution::merge(Solution::sort(l), Solution::sort(r))
    }
    fn length(mut head: &Option<Box<ListNode>>) -> usize {
        let mut ret = 0;
        while let Some(node) = head {
            ret += 1;
            head = &node.next
        }
        ret
    }
    fn merge(mut l: Option<Box<ListNode>>, mut r: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ret = ListNode::new(0);
        let mut node = &mut ret;
        while l.is_some() && r.is_some() {
            let nl = l.as_ref().map_or(0, |v| (*v).val);
            let nr = r.as_ref().map_or(0, |v| (*v).val);
            if nl <= nr {
                let tmp = l.as_mut().unwrap().next.take();
                node.next = l.take();
                l = tmp;
            } else {
                let tmp = r.as_mut().unwrap().next.take();
                node.next = r.take();
                r = tmp;
            }
            node = node.next.as_mut().unwrap();
        }
        if l.is_some() {
            node.next = l.take();
        } else {
            node.next = r.take();
        }
        ret.next
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
