use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        Solution::dfs(&root, k, &mut HashSet::new())
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, k: i32, hs: &mut HashSet<i32>) -> bool {
        if let Some(n) = node {
            let val = n.borrow().val;
            if hs.contains(&(k - val)) {
                return true;
            }
            hs.insert(val);
            if Solution::dfs(&n.borrow().left, k, hs) || Solution::dfs(&n.borrow().right, k, hs) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::find_target(
                to_tree(vec![
                    Some(5),
                    Some(3),
                    Some(6),
                    Some(2),
                    Some(4),
                    None,
                    Some(7)
                ]),
                9
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::find_target(
                to_tree(vec![
                    Some(5),
                    Some(3),
                    Some(6),
                    Some(2),
                    Some(4),
                    None,
                    Some(7)
                ]),
                28
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            true,
            Solution::find_target(to_tree(vec![Some(2), Some(1), Some(3)]), 4)
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            false,
            Solution::find_target(to_tree(vec![Some(2), Some(1), Some(3)]), 1)
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            true,
            Solution::find_target(to_tree(vec![Some(2), Some(1), Some(3)]), 3)
        );
    }
}
