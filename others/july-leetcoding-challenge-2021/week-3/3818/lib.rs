use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut node = &mut head;
        for _ in 0..k {
            if let Some(n) = node {
                node = &mut n.next;
            } else {
                return head;
            }
        }
        let mut ret = Self::reverse_k_group(node.take(), k);
        while let Some(h) = head.take() {
            ret = Some(Box::new(ListNode {
                val: h.val,
                next: ret,
            }));
            head = h.next;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            to_list(vec![2, 1, 4, 3, 5]),
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 2)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(vec![3, 2, 1, 4, 5]),
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 3)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_list(vec![1, 2, 3, 4, 5]),
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 1)
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            to_list(vec![1]),
            Solution::reverse_k_group(to_list(vec![1]), 1)
        );
    }
}
