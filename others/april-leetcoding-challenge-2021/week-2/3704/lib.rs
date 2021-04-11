use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut vd = VecDeque::new();
        if let Some(r) = root {
            vd.push_back((r, 0));
        }
        let (mut answer, mut prevdepth) = (0, None);
        while let Some((node, depth)) = vd.pop_front() {
            if Some(depth) != prevdepth {
                answer = 0;
            }
            answer += node.borrow().val;
            prevdepth = Some(depth);
            if let Some(l) = node.borrow_mut().left.take() {
                vd.push_back((l, depth + 1));
            }
            if let Some(r) = node.borrow_mut().right.take() {
                vd.push_back((r, depth + 1));
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
            15,
            Solution::deepest_leaves_sum(to_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                None,
                Some(6),
                Some(7),
                None,
                None,
                None,
                None,
                Some(8)
            ]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            19,
            Solution::deepest_leaves_sum(to_tree(vec![
                Some(6),
                Some(7),
                Some(8),
                Some(2),
                Some(7),
                Some(1),
                Some(3),
                Some(9),
                None,
                Some(1),
                Some(4),
                None,
                None,
                None,
                Some(5)
            ]))
        );
    }
}
