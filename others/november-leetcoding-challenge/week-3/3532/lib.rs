use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        Solution::dfs(&root, low, high)
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut ret = 0;
        if let Some(n) = node {
            let val = n.borrow().val;
            if low <= val && val <= high {
                ret += val;
            }
            if val >= low {
                ret += Solution::dfs(&n.borrow().left, low, high);
            }
            if val <= high {
                ret += Solution::dfs(&n.borrow().right, low, high);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        assert_eq!(
            32,
            Solution::range_sum_bst(
                to_tree(vec![
                    Some(10),
                    Some(5),
                    Some(15),
                    Some(3),
                    Some(7),
                    None,
                    Some(18)
                ]),
                7,
                15
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            23,
            Solution::range_sum_bst(
                to_tree(vec![
                    Some(10),
                    Some(5),
                    Some(15),
                    Some(3),
                    Some(7),
                    Some(13),
                    Some(18),
                    Some(1),
                    None,
                    Some(6)
                ]),
                6,
                10
            )
        );
    }
}
