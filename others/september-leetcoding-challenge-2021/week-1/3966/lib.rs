use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut answer = None;
        while let Some(h) = head.take() {
            answer = Some(Box::new(ListNode {
                val: h.val,
                next: answer,
            }));
            head = h.next;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            to_list(vec![5, 4, 3, 2, 1]),
            Solution::reverse_list(to_list(vec![1, 2, 3, 4, 5]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(vec![2, 1]),
            Solution::reverse_list(to_list(vec![1, 2]))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_list(Vec::new()),
            Solution::reverse_list(to_list(Vec::new()))
        );
    }
}
