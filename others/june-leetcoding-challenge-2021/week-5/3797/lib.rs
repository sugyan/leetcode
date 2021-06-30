use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (mut l, mut r) = (Vec::new(), Vec::new());
        if let Some(p) = p {
            Self::dfs(&root, p.borrow().val, &mut l);
        }
        if let Some(q) = q {
            Self::dfs(&root, q.borrow().val, &mut r);
        }
        let mut answer = 0;
        for i in 0..l.len().min(r.len()) {
            if l[i] == r[i] {
                answer = l[i];
            }
        }
        Self::search(&root, answer)
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, target: i32, ret: &mut Vec<i32>) -> bool {
        if let Some(n) = node {
            ret.push(n.borrow().val);
            if n.borrow().val == target {
                return true;
            }
            if Self::dfs(&n.borrow().left, target, ret) {
                return true;
            }
            if Self::dfs(&n.borrow().right, target, ret) {
                return true;
            }
            ret.pop();
        }
        false
    }
    fn search(node: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = node {
            if n.borrow().val == target {
                return Some(n.clone());
            }
            Self::search(&n.borrow().left, target)
                .or_else(|| Self::search(&n.borrow().right, target))
        } else {
            None
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
            to_tree(vec![
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(0),
                Some(8),
                None,
                None,
                Some(7),
                Some(4)
            ]),
            Solution::lowest_common_ancestor(
                to_tree(vec![
                    Some(3),
                    Some(5),
                    Some(1),
                    Some(6),
                    Some(2),
                    Some(0),
                    Some(8),
                    None,
                    None,
                    Some(7),
                    Some(4)
                ]),
                to_tree(vec![
                    Some(5),
                    Some(6),
                    Some(2),
                    None,
                    None,
                    Some(7),
                    Some(4)
                ]),
                to_tree(vec![Some(1), Some(0), Some(8)]),
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![
                Some(5),
                Some(6),
                Some(2),
                None,
                None,
                Some(7),
                Some(4)
            ]),
            Solution::lowest_common_ancestor(
                to_tree(vec![
                    Some(3),
                    Some(5),
                    Some(1),
                    Some(6),
                    Some(2),
                    Some(0),
                    Some(8),
                    None,
                    None,
                    Some(7),
                    Some(4)
                ]),
                to_tree(vec![
                    Some(5),
                    Some(6),
                    Some(2),
                    None,
                    None,
                    Some(7),
                    Some(4)
                ]),
                to_tree(vec![Some(4)]),
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_tree(vec![Some(1), Some(2)]),
            Solution::lowest_common_ancestor(
                to_tree(vec![Some(1), Some(2)]),
                to_tree(vec![Some(1), Some(2)]),
                to_tree(vec![Some(2)]),
            )
        );
    }
}
