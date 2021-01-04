use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(mut n1), Some(mut n2)) => {
                if n1.val < n2.val {
                    n1.next = Self::merge_two_lists(n1.next, Some(n2));
                    Some(n1)
                } else {
                    n2.next = Self::merge_two_lists(Some(n1), n2.next);
                    Some(n2)
                }
            }
            (Some(n1), None) => Some(n1),
            (None, Some(n2)) => Some(n2),
            (None, None) => None,
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
            to_list(vec![1, 1, 2, 3, 4, 4]),
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(vec![]),
            Solution::merge_two_lists(to_list(vec![]), to_list(vec![]))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_list(vec![0]),
            Solution::merge_two_lists(to_list(vec![]), to_list(vec![0]))
        );
    }
}
