use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut ptr = &mut dummy;
        let mut dup = &mut ptr.clone();
        for i in 1.. {
            if i == m {
                if let Some(node) = dup {
                    dup = &mut node.next;
                }
                let mut vec: Vec<i32> = Vec::new();
                for _ in 0..=n - m {
                    if let Some(node) = dup {
                        vec.push(node.val);
                        dup = &mut node.next;
                    }
                }
                let mut r = dup.clone();
                for val in vec.iter() {
                    let mut b = ListNode::new(*val);
                    b.next = r;
                    r = Some(Box::new(b));
                }
                if let Some(node) = ptr {
                    node.next = r;
                }
            }
            if let Some(node) = ptr {
                ptr = &mut node.next;
            }
            if let Some(node) = dup {
                dup = &mut node.next;
            }
            if ptr.is_none() {
                break;
            }
        }
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            to_list(vec![1, 4, 3, 2, 5]),
            Solution::reverse_between(to_list(vec![1, 2, 3, 4, 5]), 2, 4)
        );
    }
}
