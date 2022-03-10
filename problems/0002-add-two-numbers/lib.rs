use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut p = &mut dummy;
        let mut carry = false;
        while l1.is_some() || l2.is_some() || carry {
            let val = if carry { 1 } else { 0 }
                + if let Some(n1) = l1.take() {
                    l1 = n1.next;
                    n1.val
                } else {
                    0
                }
                + if let Some(n2) = l2.take() {
                    l2 = n2.next;
                    n2.val
                } else {
                    0
                };
            carry = val >= 10;
            if let Some(n) = p {
                n.next = Some(Box::new(ListNode::new(val % 10)));
                p = &mut n.next;
            }
        }
        dummy.and_then(|n| n.next)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            to_list(vec![7, 0, 8]),
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(vec![0]),
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0]))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_list(vec![8, 9, 9, 9, 0, 0, 0, 1]),
            Solution::add_two_numbers(
                to_list(vec![9, 9, 9, 9, 9, 9, 9]),
                to_list(vec![9, 9, 9, 9])
            )
        );
    }
}
