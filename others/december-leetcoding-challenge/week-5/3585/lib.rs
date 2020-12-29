use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut counts = [0; 9];
        Solution::dfs(&root, &mut counts)
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, counts: &mut [usize; 9]) -> i32 {
        if let Some(n) = node {
            let val = n.borrow().val as usize;
            let l = &n.borrow().left;
            let r = &n.borrow().right;
            counts[val - 1] += 1;
            let ret = if l.is_none() && r.is_none() {
                if counts.iter().filter(|&c| *c % 2 == 1).count() <= 1 {
                    1
                } else {
                    0
                }
            } else {
                Solution::dfs(l, counts) + Solution::dfs(r, counts)
            };
            counts[val - 1] -= 1;
            ret
        } else {
            0
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
            2,
            Solution::pseudo_palindromic_paths(to_tree(vec![
                Some(2),
                Some(3),
                Some(1),
                Some(3),
                Some(1),
                None,
                Some(1)
            ]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::pseudo_palindromic_paths(to_tree(vec![
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(3),
                None,
                None,
                None,
                None,
                None,
                Some(1)
            ]))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            1,
            Solution::pseudo_palindromic_paths(to_tree(vec![Some(9)]))
        );
    }
}
