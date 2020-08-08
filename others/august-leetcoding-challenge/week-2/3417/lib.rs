use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        Solution::helper(&root, sum)
    }
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut answer = Solution::dfs(&root, sum);
        if let Some(r) = root {
            answer += Solution::helper(&r.borrow().left, sum);
            answer += Solution::helper(&r.borrow().right, sum);
        }
        answer
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> i32 {
        let mut ret = 0;
        if let Some(n) = node {
            if n.borrow().val == target {
                ret += 1;
            }
            ret += Solution::dfs(&n.borrow().left, target - n.borrow().val);
            ret += Solution::dfs(&n.borrow().right, target - n.borrow().val);
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
            3,
            Solution::path_sum(
                to_tree(vec![
                    Some(10),
                    Some(5),
                    Some(-3),
                    Some(3),
                    Some(2),
                    None,
                    Some(11),
                    Some(3),
                    Some(-2),
                    None,
                    Some(1)
                ]),
                8
            )
        );
    }
}
