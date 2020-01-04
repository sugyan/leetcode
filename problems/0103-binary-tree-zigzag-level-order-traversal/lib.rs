use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut q: VecDeque<(Option<Rc<RefCell<TreeNode>>>, usize)> = VecDeque::new();
        q.push_back((root, 0));
        let mut answer: Vec<Vec<i32>> = Vec::new();
        while let Some(front) = q.pop_front() {
            if let Some(node) = front.0 {
                if let Some(v) = answer.get_mut(front.1) {
                    if front.1 % 2 == 0 {
                        v.push(node.borrow().val);
                    } else {
                        v.insert(0, node.borrow().val);
                    }
                } else {
                    answer.push(vec![node.borrow().val]);
                }
                q.push_back((node.borrow().left.clone(), front.1 + 1));
                q.push_back((node.borrow().right.clone(), front.1 + 1));
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
            vec![vec![3], vec![20, 9], vec![15, 7]],
            Solution::zigzag_level_order(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ]))
        )
    }
}
