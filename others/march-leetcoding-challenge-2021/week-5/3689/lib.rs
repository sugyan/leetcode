use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut answer = Vec::new();
        if Self::dfs(&root, &voyage, &mut i, &mut answer) {
            answer
        } else {
            vec![-1]
        }
    }
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        voyage: &[i32],
        i: &mut usize,
        answer: &mut Vec<i32>,
    ) -> bool {
        if let Some(n) = node {
            if n.borrow().val != voyage[*i] {
                return false;
            }
            *i += 1;
            let left = &n.borrow().left;
            if left.is_some() && left.as_ref().map(|l| l.borrow().val) != Some(voyage[*i]) {
                answer.push(n.borrow().val);
                Self::dfs(&n.borrow().right, voyage, i, answer)
                    && Self::dfs(&n.borrow().left, voyage, i, answer)
            } else {
                Self::dfs(&n.borrow().left, voyage, i, answer)
                    && Self::dfs(&n.borrow().right, voyage, i, answer)
            }
        } else {
            true
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
            vec![-1],
            Solution::flip_match_voyage(to_tree(vec![Some(1), Some(2)]), vec![2, 1])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![1],
            Solution::flip_match_voyage(to_tree(vec![Some(1), Some(2), Some(3)]), vec![1, 3, 2])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::flip_match_voyage(to_tree(vec![Some(1), Some(2), Some(3)]), vec![1, 2, 3])
        );
    }
}
