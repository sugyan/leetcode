use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }
        let mut v: Vec<&Option<Box<ListNode>>> = Vec::new();
        let mut node = &head;
        while let Some(n) = node {
            v.push(node);
            node = &n.next;
        }
        if k as usize % v.len() == 0 {
            return head;
        }
        let mut answer = v[v.len() - (k as usize) % v.len()].clone();
        let mut node = &mut answer;
        for _ in 0..v.len() - 1 {
            if let Some(n) = node {
                if n.next.is_none() {
                    n.next = v[0].clone();
                }
                node = &mut n.next;
            }
        }
        if let Some(n) = node {
            n.next = None;
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
            to_list(vec![4, 5, 1, 2, 3]),
            Solution::rotate_right(to_list(vec![1, 2, 3, 4, 5]), 2)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(vec![2, 0, 1]),
            Solution::rotate_right(to_list(vec![0, 1, 2]), 4)
        );
    }
}
