use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut vd: VecDeque<(Option<Rc<RefCell<TreeNode>>>, usize)> = VecDeque::new();
        vd.push_back((root, 0));
        let mut answer: Vec<Vec<i32>> = Vec::new();
        while let Some(e) = vd.pop_front() {
            if e.1 >= answer.len() {
                answer.push(Vec::new());
            }
            if let Some(node) = e.0 {
                answer[e.1].push(node.borrow().val);
                vd.push_back((node.borrow().left.clone(), e.1 + 1));
                vd.push_back((node.borrow().right.clone(), e.1 + 1));
            }
        }
        if answer[answer.len() - 1].is_empty() {
            answer.pop();
        }
        answer.reverse();
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
            vec![vec![15, 7], vec![9, 20], vec![3]],
            Solution::level_order_bottom(to_tree(vec![
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
}
