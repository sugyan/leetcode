use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut n1 = &l1;
        let mut n2 = &l2;
        let mut ret = None;
        let mut p = &mut ret;
        let mut carry = false;
        while n1.is_some() || n2.is_some() || carry {
            let v1 = if let Some(n) = n1 {
                n1 = &n.next;
                n.val
            } else {
                0
            };
            let v2 = if let Some(n) = n2 {
                n2 = &n.next;
                n.val
            } else {
                0
            };
            let mut v = v1 + v2 + if carry { 1 } else { 0 };
            carry = if v > 9 {
                v -= 10;
                true
            } else {
                false
            };
            *p = Some(Box::new(ListNode::new(v)));
            if let Some(pp) = p {
                p = &mut pp.next;
            }
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
