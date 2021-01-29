use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut btm = BTreeMap::new();
        Self::traverse(&root, &mut btm, (0, 0));
        btm.values_mut()
            .map(|bh| {
                (0..bh.len())
                    .filter_map(|_| bh.pop().map(|(_, Reverse(i))| i))
                    .collect()
            })
            .collect()
    }
    fn traverse(
        node: &Option<Rc<RefCell<TreeNode>>>,
        btm: &mut BTreeMap<i32, BinaryHeap<(i32, Reverse<i32>)>>,
        pos: (i32, i32),
    ) {
        if let Some(n) = node {
            btm.entry(pos.0)
                .or_insert_with(BinaryHeap::new)
                .push((pos.1, Reverse(n.borrow().val)));
            Self::traverse(&n.borrow().left, btm, (pos.0 - 1, pos.1 - 1));
            Self::traverse(&n.borrow().right, btm, (pos.0 + 1, pos.1 - 1));
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
            vec![vec![9], vec![3, 15], vec![20], vec![7]],
            Solution::vertical_traversal(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ]))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]],
            Solution::vertical_traversal(to_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7)
            ]))
        );
    }
}
