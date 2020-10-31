use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut prev: Option<i32> = None;
        let mut v: Vec<(i32, i32)> = Vec::new();
        Solution::find_targets(root, &mut prev, &mut v);
        match v.len() {
            1 => Solution::swap(root, &v[0]),
            2 => Solution::swap(root, &(v[0].0, v[1].1)),
            _ => {}
        }
    }
    fn find_targets(
        node: &mut Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<i32>,
        v: &mut Vec<(i32, i32)>,
    ) {
        if let Some(n) = node {
            Solution::find_targets(&mut n.borrow_mut().left, prev, v);
            let val = n.borrow().val;
            if let Some(p) = *prev {
                if p > val {
                    v.push((p, val));
                }
            }
            *prev = Some(val);
            Solution::find_targets(&mut n.borrow_mut().right, prev, v);
        }
    }
    fn swap(node: &mut Option<Rc<RefCell<TreeNode>>>, target: &(i32, i32)) {
        if let Some(n) = node {
            let val = n.borrow().val;
            if val == target.0 {
                n.borrow_mut().val = target.1;
            }
            if val == target.1 {
                n.borrow_mut().val = target.0;
            }
            Solution::swap(&mut n.borrow_mut().left, target);
            Solution::swap(&mut n.borrow_mut().right, target);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        let mut root = to_tree(vec![Some(1), Some(3), None, None, Some(2)]);
        Solution::recover_tree(&mut root);
        assert_eq!(to_tree(vec![Some(3), Some(1), None, None, Some(2)]), root);
    }

    #[test]
    fn example_2() {
        let mut root = to_tree(vec![Some(3), Some(1), Some(4), None, None, Some(2)]);
        Solution::recover_tree(&mut root);
        assert_eq!(
            to_tree(vec![Some(2), Some(1), Some(4), None, None, Some(3)]),
            root
        );
    }
}
