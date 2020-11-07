use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut v1 = Solution::list2vec(&l1);
        let mut v2 = Solution::list2vec(&l2);
        v1.reverse();
        v2.reverse();
        let mut next = None;
        let mut carry = 0;
        for i in 0..=std::cmp::max(v1.len(), v2.len()) {
            let mut val =
                carry + if i < v1.len() { v1[i] } else { 0 } + if i < v2.len() { v2[i] } else { 0 };
            carry = if val > 9 {
                val -= 10;
                1
            } else {
                0
            };
            if i < std::cmp::max(v1.len(), v2.len()) || val != 0 {
                next = Some(Box::new(ListNode { val, next }));
            }
        }
        next
    }
    fn list2vec(list: &Option<Box<ListNode>>) -> Vec<i32> {
        if list.is_none() {
            return vec![0];
        }
        let mut ret = Vec::new();
        let mut node = list;
        while let Some(n) = node {
            ret.push(n.val);
            node = &n.next;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            to_list(vec![7, 8, 0, 7]),
            Solution::add_two_numbers(to_list(vec![7, 2, 4, 3]), to_list(vec![5, 6, 4]))
        );
    }
}
