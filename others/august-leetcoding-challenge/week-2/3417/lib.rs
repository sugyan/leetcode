use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut answer = 0;
        Solution::dfs(&root, sum, &mut answer);
        answer
    }
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        target: i32,
        answer: &mut i32,
    ) -> HashMap<i32, i32> {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        if let Some(n) = node {
            *hm.entry(n.borrow().val).or_default() += 1;
            for (k, v) in Solution::dfs(&n.borrow().left, target, answer).iter() {
                *hm.entry(n.borrow().val + k).or_default() += v;
            }
            for (k, v) in Solution::dfs(&n.borrow().right, target, answer).iter() {
                *hm.entry(n.borrow().val + k).or_default() += v;
            }
        }
        if let Some(val) = hm.get(&target) {
            *answer += val;
        }
        hm
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
