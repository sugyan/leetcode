use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut node = &mut dummy.as_mut().unwrap().next;
        let mut remove = None;
        loop {
            match node {
                Some(n) if Some(n.val) == remove => {
                    *node = n.next.take();
                }
                Some(n) if n.next.is_some() && n.next.as_ref().unwrap().val == n.val => {
                    remove = Some(n.val);
                }
                Some(n) => {
                    remove = None;
                    node = &mut n.next;
                }
                None => return dummy.unwrap().next,
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
        assert_eq!(
            to_list(vec![1, 2, 5]),
            Solution::delete_duplicates(to_list(vec![1, 2, 3, 3, 4, 4, 5]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(vec![2, 3]),
            Solution::delete_duplicates(to_list(vec![1, 1, 1, 2, 3]))
        );
    }
}
