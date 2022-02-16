use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut node| {
            if let Some(mut n) = node.next {
                node.next = Self::swap_pairs(n.next);
                n.next = Some(node);
                n
            } else {
                node
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            to_list(vec![2, 1, 4, 3]),
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(to_list(vec![]), Solution::swap_pairs(to_list(vec![])));
    }

    #[test]
    fn example_3() {
        assert_eq!(to_list(vec![1]), Solution::swap_pairs(to_list(vec![1])));
    }
}
