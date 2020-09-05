use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut v1: Vec<i32> = Vec::new();
        let mut v2: Vec<i32> = Vec::new();
        Solution::dfs(&root1, &mut v1);
        Solution::dfs(&root2, &mut v2);
        let mut answer: Vec<i32> = Vec::new();
        let (mut i1, mut i2) = (0, 0);
        while i1 < v1.len() && i2 < v2.len() {
            if v1[i1] < v2[i2] {
                answer.push(v1[i1]);
                i1 += 1;
            } else {
                answer.push(v2[i2]);
                i2 += 1;
            }
        }
        answer.extend(&v1[i1..]);
        answer.extend(&v2[i2..]);
        answer
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(n) = node {
            Solution::dfs(&n.borrow().left, v);
            v.push(n.borrow().val);
            Solution::dfs(&n.borrow().right, v);
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
            vec![0, 1, 1, 2, 3, 4],
            Solution::get_all_elements(
                to_tree(vec![Some(2), Some(1), Some(4)]),
                to_tree(vec![Some(1), Some(0), Some(3)])
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![-10, 0, 0, 1, 2, 5, 7, 10],
            Solution::get_all_elements(
                to_tree(vec![Some(0), Some(-10), Some(10)]),
                to_tree(vec![Some(5), Some(1), Some(7), Some(0), Some(2)])
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec![0, 1, 2, 5, 7],
            Solution::get_all_elements(
                None,
                to_tree(vec![Some(5), Some(1), Some(7), Some(0), Some(2)])
            )
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            vec![-10, 0, 10],
            Solution::get_all_elements(to_tree(vec![Some(0), Some(-10), Some(10)]), None)
        );
    }
}
