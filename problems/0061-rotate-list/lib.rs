use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut v = Vec::new();
        let mut node = &head;
        while let Some(n) = node {
            v.push(n.val);
            node = &n.next;
        }
        let mut answer = None;
        for i in (0..v.len()).rev() {
            let j = k as usize % v.len();
            answer = Some(Box::new(ListNode {
                val: v[(v.len() + i - j) % v.len()],
                next: answer,
            }))
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
