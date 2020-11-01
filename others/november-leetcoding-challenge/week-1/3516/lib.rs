use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut answer = 0;
        let mut node = &head;
        while let Some(n) = node {
            answer *= 2;
            if n.val == 1 {
                answer += 1;
            }
            node = &n.next;
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
        assert_eq!(5, Solution::get_decimal_value(to_list(vec![1, 0, 1])));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::get_decimal_value(to_list(vec![0])));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::get_decimal_value(to_list(vec![1])));
    }

    #[test]
    fn example_4() {
        assert_eq!(
            18880,
            Solution::get_decimal_value(to_list(vec![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]))
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(0, Solution::get_decimal_value(to_list(vec![0, 0])));
    }
}
