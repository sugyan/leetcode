use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(n1), Some(n2)) => Some(Box::new(if n1.val < n2.val {
                ListNode {
                    val: n1.val,
                    next: Self::merge_two_lists(n1.next, Some(n2)),
                }
            } else {
                ListNode {
                    val: n2.val,
                    next: Self::merge_two_lists(Some(n1), n2.next),
                }
            })),
            (Some(n), None) | (None, Some(n)) => Some(n),
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
