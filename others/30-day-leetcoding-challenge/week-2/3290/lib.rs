use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut node = &mut head;
        let mut len = 0;
        while let Some(n) = node {
            len += 1;
            node = &mut n.next;
        }
        len /= 2;
        node = &mut head;
        while let Some(n) = node {
            if len == 0 {
                return Some(n.clone());
            }
            len -= 1;
            node = &mut n.next;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            to_list(vec![3, 4, 5]),
            Solution::middle_node(to_list(vec![1, 2, 3, 4, 5]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(vec![4, 5, 6]),
            Solution::middle_node(to_list(vec![1, 2, 3, 4, 5, 6]))
        );
    }
}
