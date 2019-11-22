use utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut answer = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut answer;
        let mut carry = false;
        loop {
            if !carry && l1.is_none() && l2.is_none() {
                break;
            }
            let mut d = 0;
            d += if let Some(l) = l1 {
                l1 = l.next;
                l.val
            } else {
                0
            };
            d += if let Some(l) = l2 {
                l2 = l.next;
                l.val
            } else {
                0
            };
            d += if carry { 1 } else { 0 };
            carry = d >= 10;
            if carry {
                d -= 10;
            }
            if let Some(t) = tail {
                t.next = Some(Box::new(ListNode::new(d)));
                tail = &mut t.next;
            }
        }
        return answer.unwrap().next;
    }
}

fn main() {
    let l1 = utils::readline_to_list();
    let l2 = utils::readline_to_list();
    println!(
        "{:?}",
        utils::list_to_vec(Solution::add_two_numbers(l1, l2))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_list;

    #[test]
    fn example_1() {
        let l1: Option<Box<ListNode>> = to_list(vec![2, 4, 3]);
        let l2: Option<Box<ListNode>> = to_list(vec![5, 6, 4]);
        assert_eq!(to_list(vec![7, 0, 8]), Solution::add_two_numbers(l1, l2));
    }
}
