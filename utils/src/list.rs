#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in v.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

pub fn list_to_vec(l: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let mut node = &mut l.clone();
    while let Some(n) = node {
        v.push(n.val);
        node = &mut n.next;
    }
    return v;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_list() {
        assert_eq!(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
            to_list(vec![1, 2, 3])
        );
    }
}
