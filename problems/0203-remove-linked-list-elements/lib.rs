use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut node = &mut head;
        loop {
            match node {
                Some(n) if n.val == val => {
                    *node = n.next.take();
                }
                Some(n) => {
                    node = &mut n.next;
                }
                None => break,
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            to_list(vec![1, 2, 3, 4, 5]),
            Solution::remove_elements(to_list(vec![1, 2, 6, 3, 4, 5, 6]), 6)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(Vec::new()),
            Solution::remove_elements(to_list(Vec::new()), 1)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_list(Vec::new()),
            Solution::remove_elements(to_list(vec![7, 7, 7, 7]), 7)
        );
    }
}
