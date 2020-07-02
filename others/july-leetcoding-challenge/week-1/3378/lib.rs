use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        Solution::dfs(&root, 0, &mut answer);
        answer.reverse();
        answer
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, answer: &mut Vec<Vec<i32>>) {
        if let Some(n) = node {
            if depth == answer.len() {
                answer.push(Vec::new());
            }
            answer[depth].push(n.borrow().val);
            Solution::dfs(&n.borrow().left, depth + 1, answer);
            Solution::dfs(&n.borrow().right, depth + 1, answer);
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
            vec![vec![15, 7], vec![9, 20], vec![3]],
            Solution::level_order_bottom(to_tree(vec![
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
