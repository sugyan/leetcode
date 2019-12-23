use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node = ListNode::new(0);
        node.next = head;
        let mut dummy = Some(Box::new(node));
        let mut ptr = &mut dummy;
        loop {
            let mut repeat = false;
            if let Some(n) = ptr {
                if let Some(n1) = &n.next {
                    if let Some(n2) = &n1.next {
                        if n1.val == n2.val {
                            let mut next = &mut Some(n2.clone());
                            while let Some(node) = next {
                                if node.val != n1.val {
                                    n.next = Some(node.clone());
                                    break;
                                } else if node.next == None {
                                    n.next = None;
                                    break;
                                }
                                next = &mut node.next;
                            }
                            repeat = true;
                        }
                    }
                }
            } else {
                break;
            }
            if !repeat {
                if let Some(n) = ptr {
                    ptr = &mut n.next;
                }
            }
        }
        return dummy.unwrap().next;
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
