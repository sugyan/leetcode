use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();
        let mut d: VecDeque<(usize, Rc<RefCell<TreeNode>>)> = VecDeque::new();
        if let Some(r) = root {
            d.push_back((0, r));
        }
        while let Some(f) = d.pop_front() {
            if f.0 < answer.len() {
                answer[f.0] = f.1.borrow().val;
            } else {
                answer.push(f.1.borrow().val);
            }
            if let Some(l) = &f.1.borrow().left {
                d.push_back((f.0 + 1, l.clone()));
            }
            if let Some(r) = &f.1.borrow().right {
                d.push_back((f.0 + 1, r.clone()));
            }
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
            vec![1, 3, 4],
            Solution::right_side_view(to_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                None,
                Some(5),
                None,
                Some(4)
            ]))
        );
    }
}
