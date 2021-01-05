use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        Self::recursive(&mut dummy);
        dummy.next
    }
    fn recursive(node: &mut Box<ListNode>) {
        if let Some(mut n) = node.next.take() {
            let mut removed = false;
            while let Some(next) = n.next.take() {
                if next.val == n.val {
                    removed = true;
                    n = next;
                } else {
                    n.next = Some(next);
                    break;
                }
            }
            Self::recursive(&mut n);
            node.next = if removed { n.next } else { Some(n) };
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
