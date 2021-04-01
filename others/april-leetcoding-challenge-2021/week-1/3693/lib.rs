use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut len = 0;
        let mut node = &head;
        while let Some(n) = node {
            len += 1;
            node = &n.next;
        }
        let mut head = head;
        let second = {
            let mut node = &mut head;
            for _ in 0..len / 2 + len % 2 {
                if let Some(n) = node {
                    node = &mut n.next;
                }
            }
            node.take()
        };
        let mut first = None;
        for _ in 0..len / 2 {
            if let Some(mut node) = head.take() {
                let next = node.next.take();
                node.next = first.take();
                first = Some(node);
                head = next;
            }
        }
        first == second
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::is_palindrome(to_list(vec![1, 2, 2, 1])));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::is_palindrome(to_list(vec![1, 2])));
    }
}
