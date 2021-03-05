use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut vd = VecDeque::new();
        if let Some(r) = root {
            vd.push_back(r);
        }
        let mut answer = Vec::new();
        while !vd.is_empty() {
            let len = vd.len();
            let mut sum = 0;
            for _ in 0..vd.len() {
                if let Some(node) = vd.pop_front() {
                    sum += i64::from(node.borrow().val);
                    if let Some(n) = node.borrow_mut().left.take() {
                        vd.push_back(n);
                    }
                    if let Some(n) = node.borrow_mut().right.take() {
                        vd.push_back(n);
                    }
                }
            }
            answer.push(sum as f64 / len as f64);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        let ret = Solution::average_of_levels(to_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]));
        assert_eq!(3, ret.len());
        assert!(vec![3.0, 14.5, 11.0]
            .iter()
            .zip(ret.iter())
            .all(|(expected, actual)| (expected - actual).abs() < std::f64::EPSILON));
    }
}
