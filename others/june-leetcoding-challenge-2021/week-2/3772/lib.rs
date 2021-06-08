use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let hm = inorder
            .iter()
            .enumerate()
            .map(|(i, &val)| (val, i))
            .collect::<HashMap<_, _>>();
        Self::helper(&mut preorder.iter(), &hm, (0, preorder.len() as isize - 1))
    }
    fn helper(
        preorder: &mut std::slice::Iter<i32>,
        index_map: &HashMap<i32, usize>,
        range: (isize, isize),
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if range.0 <= range.1 {
            if let Some(&val) = preorder.next() {
                if let Some(&i) = index_map.get(&val) {
                    return Some(Rc::new(RefCell::new(TreeNode {
                        val,
                        left: Self::helper(preorder, index_map, (range.0, i as isize - 1)),
                        right: Self::helper(preorder, index_map, (i as isize + 1, range.1)),
                    })));
                }
            }
        }
        None
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
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ]),
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![Some(-1),]),
            Solution::build_tree(vec![-1], vec![-1])
        )
    }
}
