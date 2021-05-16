use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let states = Self::dfs(&root);
        states.1.min(states.2)
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if let Some(n) = node {
            let l = Self::dfs(&n.borrow().left);
            let r = Self::dfs(&n.borrow().right);
            let minl = l.1.min(l.2);
            let minr = r.1.min(r.2);
            (
                l.1.saturating_add(r.1),
                (l.2.saturating_add(minr)).min(r.2.saturating_add(minl)),
                1 + l.0.min(minl) + r.0.min(minr),
            )
        } else {
            (0, 0, std::i32::MAX)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        assert_eq!(
            1,
            Solution::min_camera_cover(to_tree(vec![Some(0), Some(0), None, Some(0), Some(0)]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            2,
            Solution::min_camera_cover(to_tree(vec![
                Some(0),
                Some(0),
                None,
                Some(0),
                None,
                Some(0),
                None,
                None,
                Some(0)
            ]))
        );
    }
}
