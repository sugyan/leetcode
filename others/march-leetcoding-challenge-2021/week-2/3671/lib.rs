use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut v = Vec::new();
        {
            let mut node = &head;
            while let Some(n) = node {
                v.push(n.val);
                node = &n.next;
            }
        }
        let len = v.len();
        v.swap(k as usize - 1, len - k as usize);
        let mut answer = None;
        for &n in v.iter().rev() {
            answer = Some(Box::new(ListNode {
                val: n,
                next: answer,
            }));
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
            Solution::swap_nodes(to_list(vec![1, 2, 3, 4, 5]), 2)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(vec![7, 9, 6, 6, 8, 7, 3, 0, 9, 5]),
            Solution::swap_nodes(to_list(vec![7, 9, 6, 6, 7, 8, 3, 0, 9, 5]), 5)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(to_list(vec![1]), Solution::swap_nodes(to_list(vec![1]), 1));
    }

    #[test]
    fn example_4() {
        assert_eq!(
            to_list(vec![2, 1]),
            Solution::swap_nodes(to_list(vec![1, 2]), 1)
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            to_list(vec![1, 2, 3]),
            Solution::swap_nodes(to_list(vec![1, 2, 3]), 2)
        );
    }
}
