use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut btm: BTreeMap<i32, BinaryHeap<Reverse<(i32, i32)>>> = BTreeMap::new();
        Solution::dfs(&root, (0, 0), &mut btm);
        let mut answer: Vec<Vec<i32>> = Vec::new();
        for bh in btm.values_mut() {
            let mut v: Vec<i32> = Vec::new();
            while let Some(r) = bh.pop() {
                v.push((r.0).1);
            }
            answer.push(v);
        }
        answer
    }
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        pos: (i32, i32),
        btm: &mut BTreeMap<i32, BinaryHeap<Reverse<(i32, i32)>>>,
    ) {
        if let Some(n) = node {
            btm.entry(pos.0)
                .or_insert_with(BinaryHeap::new)
                .push(Reverse((pos.1, n.borrow().val)));
            Solution::dfs(&n.borrow().left, (pos.0 - 1, pos.1 + 1), btm);
            Solution::dfs(&n.borrow().right, (pos.0 + 1, pos.1 + 1), btm);
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
