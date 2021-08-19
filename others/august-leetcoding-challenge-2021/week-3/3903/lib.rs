use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sums = Vec::new();
        Self::dfs(&root, &mut sums);
        ((0..sums.len() - 1)
            .map(|i| sums[i] * (sums[sums.len() - 1] - sums[i]))
            .max()
            .unwrap()
            % MOD) as i32
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, sums: &mut Vec<i64>) -> i64 {
        if let Some(n) = node {
            let sum = n.borrow().val as i64
                + Self::dfs(&n.borrow().left, sums)
                + Self::dfs(&n.borrow().right, sums);
            sums.push(sum);
            sum
        } else {
            0
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
            110,
            Solution::max_product(to_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6)
            ]))
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            90,
            Solution::max_product(to_tree(vec![
                Some(1),
                None,
                Some(2),
                Some(3),
                Some(4),
                None,
                None,
                Some(5),
                Some(6)
            ]))
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            1025,
            Solution::max_product(to_tree(vec![
                Some(2),
                Some(3),
                Some(9),
                Some(10),
                Some(7),
                Some(8),
                Some(6),
                Some(5),
                Some(4),
                Some(11),
                Some(1)
            ]))
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(1, Solution::max_product(to_tree(vec![Some(1), Some(1)])));
    }
}
