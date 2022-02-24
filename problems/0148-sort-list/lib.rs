use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut first, mut second) = (None, None);
        let mut flag = false;
        while let Some(mut node) = head {
            let n = node.next.take();
            if flag {
                node.next = first.take();
                first = Some(node);
            } else {
                node.next = second.take();
                second = Some(node);
            }
            head = n;
            flag = !flag;
        }
        match (first, second) {
            (None, None) => None,
            (node, None) => node,
            (None, node) => node,
            (f, s) => Self::merge(Self::sort_list(f), Self::sort_list(s)),
        }
    }
    fn merge(first: Option<Box<ListNode>>, second: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (first, second) {
            (None, None) => None,
            (node, None) => node,
            (None, node) => node,
            (Some(nf), Some(ns)) => {
                let (mut small, large) = if nf.val < ns.val { (nf, ns) } else { (ns, nf) };
                let s = small.next.take();
                small.next = Self::merge(s, Some(large));
                Some(small)
            }
        }
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
