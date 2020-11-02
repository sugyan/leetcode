use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut dummy = ListNode::new(0);
        while let Some(mut node) = head {
            let mut ptr = &mut dummy;
            while ptr.next.is_some() && ptr.next.as_ref().unwrap().val < node.val {
                ptr = ptr.next.as_mut().unwrap();
            }
            let next = node.next;
            node.next = ptr.next.take();
            ptr.next = Some(node);
            head = next;
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
            to_list(vec![1, 2, 3, 4]),
            Solution::insertion_sort_list(to_list(vec![4, 2, 1, 3]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(vec![-1, 0, 3, 4, 5]),
            Solution::insertion_sort_list(to_list(vec![-1, 5, 3, 4, 0]))
        );
    }
}
