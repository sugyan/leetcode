use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        Self::dfs(
            &root,
            0,
            target_sum,
            &mut vec![(0, 1)].into_iter().collect(),
        )
    }
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        current: i32,
        target_sum: i32,
        hm: &mut HashMap<i32, i32>,
    ) -> i32 {
        let mut ret = 0;
        if let Some(n) = node {
            let sum = current + n.borrow().val;
            ret += hm.get(&(sum - target_sum)).unwrap_or(&0);
            *hm.entry(sum).or_default() += 1;
            ret += Self::dfs(&n.borrow().left, sum, target_sum, hm);
            ret += Self::dfs(&n.borrow().right, sum, target_sum, hm);
            *hm.entry(sum).or_default() -= 1;
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

    #[test]
    fn example_2() {
        assert_eq!(
            3,
            Solution::path_sum(
                to_tree(vec![
                    Some(5),
                    Some(4),
                    Some(8),
                    Some(11),
                    None,
                    Some(13),
                    Some(4),
                    Some(7),
                    Some(2),
                    None,
                    None,
                    Some(5),
                    Some(1)
                ]),
                22
            )
        );
    }
}
