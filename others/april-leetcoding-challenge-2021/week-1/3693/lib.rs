use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut node = &head;
        let mut v = Vec::new();
        while let Some(n) = node {
            v.push(n.val);
            node = &n.next;
        }
        (0..v.len() / 2).all(|i| v[i] == v[v.len() - 1 - i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::is_palindrome(to_list(vec![1, 2, 2, 1])));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::is_palindrome(to_list(vec![1, 2])));
    }
}
