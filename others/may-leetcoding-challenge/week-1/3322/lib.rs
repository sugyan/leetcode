use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let dx = Solution::dfs(0, x, &root, None).unwrap();
        let dy = Solution::dfs(0, y, &root, None).unwrap();
        dx.0 == dy.0 && dx.1 != dy.1
    }

    fn dfs(
        depth: usize,
        target: i32,
        node: &Option<Rc<RefCell<TreeNode>>>,
        parent: Option<i32>,
    ) -> Option<(usize, Option<i32>)> {
        if let Some(n) = node {
            let val = n.borrow().val;
            if val == target {
                return Some((depth, parent));
            }
            let l = Solution::dfs(depth + 1, target, &n.borrow().left, Some(val));
            if l.is_some() {
                return l;
            }
            let r = Solution::dfs(depth + 1, target, &n.borrow().right, Some(val));
            if r.is_some() {
                return r;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        assert_eq!(
            false,
            Solution::is_cousins(to_tree(vec![Some(1), Some(2), Some(3), Some(4)]), 4, 3)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            true,
            Solution::is_cousins(
                to_tree(vec![
                    Some(1),
                    Some(2),
                    Some(3),
                    None,
                    Some(4),
                    None,
                    Some(5)
                ]),
                5,
                4
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::is_cousins(
                to_tree(vec![Some(1), Some(2), Some(3), None, Some(4)]),
                2,
                3
            )
        );
    }
}
