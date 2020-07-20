use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut v: Vec<i32> = Vec::new();
        let mut node: &Option<Box<ListNode>> = &head;
        while let Some(n) = node {
            if n.val != val {
                v.push(n.val);
            }
            node = &n.next;
        }
        let mut answer: Option<Box<ListNode>> = None;
        for &e in v.iter().rev() {
            let mut node = ListNode::new(e);
            node.next = answer;
            answer = Some(Box::new(node));
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
            to_list(vec![1, 2, 3, 4, 5]),
            Solution::remove_elements(to_list(vec![1, 2, 6, 3, 4, 5, 6]), 6)
        );
    }
}
