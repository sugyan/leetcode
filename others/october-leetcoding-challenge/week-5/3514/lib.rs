use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut prev: Option<i32> = None;
        let mut swapped: Option<(i32, i32)> = None;
        Solution::find_targets(root, &mut prev, &mut swapped);
        if let Some(s) = &swapped {
            Solution::swap(root, *s);
        }
    }
    fn find_targets(
        node: &mut Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<i32>,
        swapped: &mut Option<(i32, i32)>,
    ) {
        if let Some(n) = node {
            Solution::find_targets(&mut n.borrow_mut().left, prev, swapped);
            let val = n.borrow().val;
            if let Some(p) = *prev {
                if p > val {
                    if let Some(s) = swapped {
                        (*s).1 = val;
                    } else {
                        *swapped = Some((p, val));
                    }
                }
            }
            *prev = Some(val);
            Solution::find_targets(&mut n.borrow_mut().right, prev, swapped);
        }
    }
    fn swap(node: &mut Option<Rc<RefCell<TreeNode>>>, target: (i32, i32)) {
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
