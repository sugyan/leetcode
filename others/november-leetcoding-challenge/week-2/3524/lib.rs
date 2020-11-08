use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(&root).0
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(n) = node {
            let l = Solution::dfs(&n.borrow().left);
            let r = Solution::dfs(&n.borrow().right);
            ((l.1 - r.1).abs() + l.0 + r.0, n.borrow().val + l.1 + r.1)
        } else {
            (0, 0)
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
            Solution::find_tilt(to_tree(vec![Some(1), Some(2), Some(3)]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            15,
            Solution::find_tilt(to_tree(vec![
                Some(4),
                Some(2),
                Some(9),
                Some(3),
                Some(5),
                None,
                Some(7)
            ]))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            9,
            Solution::find_tilt(to_tree(vec![
                Some(21),
                Some(7),
                Some(14),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(3)
            ]))
        );
    }
}
