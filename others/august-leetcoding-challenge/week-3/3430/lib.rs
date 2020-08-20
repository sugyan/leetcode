use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut v: Vec<i32> = Vec::new();
        {
            let mut node = head.clone();
            while let Some(n) = node {
                v.push(n.val);
                node = n.next;
            }
        }
        for i in 0..v.len() {
            if i % 2 == 1 {
                if let Some(n) = v.pop() {
                    v.insert(i, n);
                }
            }
        }
        {
            let mut node = head;
            while let Some(n) = node {
                n.val = v.remove(0);
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
