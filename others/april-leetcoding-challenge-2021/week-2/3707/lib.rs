use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let (mut lt, mut gt) = (Vec::new(), Vec::new());
        let mut node = &head;
        while let Some(n) = node {
            if n.val < x {
                lt.push(n.val);
            } else {
                gt.push(n.val);
            }
            node = &n.next;
        }
        let mut answer = None;
        for &n in gt.iter().rev().chain(lt.iter().rev()) {
            answer = Some(Box::new(ListNode {
                val: n,
                next: answer,
            }));
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
