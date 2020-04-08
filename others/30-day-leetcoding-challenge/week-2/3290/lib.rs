use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;
        let mut flg = true;
        while let Some(f) = fast {
            fast = &f.next;
            flg = !flg;
            if flg {
                if let Some(s) = slow {
                    slow = &s.next;
                }
            }
        }
        slow.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            to_list(vec![3, 4, 5]),
            Solution::middle_node(to_list(vec![1, 2, 3, 4, 5]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_list(vec![4, 5, 6]),
            Solution::middle_node(to_list(vec![1, 2, 3, 4, 5, 6]))
        );
    }
}
