use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut answer = Vec::new();
        Self::dfs(&root, target_sum, &mut Vec::new(), &mut answer);
        answer
    }
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        v: &mut Vec<i32>,
        answer: &mut Vec<Vec<i32>>,
    ) {
        if let Some(n) = node {
            let val = n.borrow().val;
            let l = &n.borrow().left;
            let r = &n.borrow().right;
            v.push(val);
            if target_sum == val && l.is_none() && r.is_none() {
                answer.push(v.clone());
            }
            Self::dfs(l, target_sum - val, v, answer);
            Self::dfs(r, target_sum - val, v, answer);
            v.pop();
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
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]],
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

    #[test]
    fn example_2() {
        assert_eq!(
            Vec::<Vec<i32>>::new(),
            Solution::path_sum(to_tree(vec![Some(1), Some(2), Some(3)]), 5)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Vec::<Vec<i32>>::new(),
            Solution::path_sum(to_tree(vec![Some(1), Some(2)]), 0)
        );
    }
}
