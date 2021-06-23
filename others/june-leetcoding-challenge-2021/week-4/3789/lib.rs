use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut v = Vec::new();
        let mut node = &head;
        while let Some(n) = node {
            v.push(n.val);
            node = &n.next;
        }
        for i in 0..=(right - left) as usize / 2 {
            v.swap(left as usize - 1 + i, right as usize - 1 - i);
        }
        let mut answer = None;
        for &val in v.iter().rev() {
            answer = Some(Box::new(ListNode { val, next: answer }));
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
            to_list(vec![1, 4, 3, 2, 5]),
            Solution::reverse_between(to_list(vec![1, 2, 3, 4, 5]), 2, 4)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(vec![5]),
            Solution::reverse_between(to_list(vec![5]), 1, 1)
        );
    }
}
