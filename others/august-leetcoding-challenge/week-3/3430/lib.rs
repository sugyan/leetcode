use std::collections::VecDeque;
use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut vd: VecDeque<i32> = VecDeque::new();
        {
            let mut node = head.clone();
            while let Some(n) = node {
                vd.push_back(n.val);
                node = n.next;
            }
        }
        {
            let mut node = head;
            let mut i = 0;
            while let Some(n) = node {
                n.val = if i % 2 == 0 {
                    vd.pop_front().unwrap()
                } else {
                    vd.pop_back().unwrap()
                };
                i += 1;
                node = &mut n.next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        let mut list = to_list(vec![1, 2, 3, 4]);
        Solution::reorder_list(&mut list);
        assert_eq!(to_list(vec![1, 4, 2, 3]), list);
    }

    #[test]
    fn example_2() {
        let mut list = to_list(vec![1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut list);
        assert_eq!(to_list(vec![1, 5, 2, 4, 3]), list);
    }
}
