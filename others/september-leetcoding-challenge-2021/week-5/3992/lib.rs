use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut len = 0;
        let mut node = &head;
        while let Some(n) = node {
            len += 1;
            node = &n.next;
        }
        let mut head = head;
        let mut answer = Vec::with_capacity(k as usize);
        for i in 0..k as usize {
            answer.push(head);
            let mut node = &mut answer[i];
            for _ in 0..len / k + if i < (len % k) as usize { 1 } else { 0 } {
                if let Some(n) = node {
                    node = &mut n.next;
                }
            }
            head = node.take();
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
            vec![
                to_list(vec![1]),
                to_list(vec![2]),
                to_list(vec![3]),
                to_list(vec![]),
                to_list(vec![])
            ],
            Solution::split_list_to_parts(to_list(vec![1, 2, 3]), 5)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![
                to_list(vec![1, 2, 3, 4]),
                to_list(vec![5, 6, 7]),
                to_list(vec![8, 9, 10]),
            ],
            Solution::split_list_to_parts(to_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 3)
        );
    }
}
