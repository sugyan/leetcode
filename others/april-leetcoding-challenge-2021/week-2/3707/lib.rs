use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut lt = Box::new(ListNode::new(0));
        let mut ge = Box::new(ListNode::new(0));
        let mut plt = &mut lt;
        let mut pge = &mut ge;
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                plt.next = Some(node);
                plt = plt.next.as_mut().unwrap();
            } else {
                pge.next = Some(node);
                pge = pge.next.as_mut().unwrap();
            }
        }
        plt.next = ge.next;
        lt.next
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

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(vec![1, 2]),
            Solution::partition(to_list(vec![2, 1]), 2)
        );
    }
}
