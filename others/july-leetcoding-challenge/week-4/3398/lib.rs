use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        Solution::dfs(&root, &mut answer, 0);
        answer
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, answer: &mut Vec<Vec<i32>>, depth: usize) {
        if let Some(n) = node {
            if answer.len() == depth {
                answer.push(Vec::new());
            }
            if depth % 2 == 0 {
                answer[depth].push(n.borrow().val);
            } else {
                answer[depth].insert(0, n.borrow().val);
            }
            Solution::dfs(&n.borrow().left, answer, depth + 1);
            Solution::dfs(&n.borrow().right, answer, depth + 1);
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
            vec![vec![3], vec![20, 9], vec![15, 7]],
            Solution::zigzag_level_order(to_tree(vec![
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
}
