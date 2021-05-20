use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut vd = VecDeque::new();
        if let Some(r) = root {
            vd.push_back(r);
        }
        let mut answer = Vec::new();
        while !vd.is_empty() {
            let mut row = Vec::new();
            for _ in 0..vd.len() {
                if let Some(node) = vd.pop_front() {
                    row.push(node.borrow().val);
                    if let Some(l) = node.borrow_mut().left.take() {
                        vd.push_back(l);
                    }
                    if let Some(r) = node.borrow_mut().right.take() {
                        vd.push_back(r);
                    }
                }
            }
            answer.push(row);
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
        assert_eq!(
            vec![vec![3], vec![9, 20], vec![15, 7]],
            Solution::level_order(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![vec![1]], Solution::level_order(to_tree(vec![Some(1)])));
    }

    #[test]
    fn example_3() {
        assert_eq!(Vec::<Vec<i32>>::new(), Solution::level_order(None));
    }
}
