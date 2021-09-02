use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::helper(&(1..=n).collect::<Vec<_>>())
    }
    fn helper(v: &[i32]) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if v.is_empty() {
            return vec![None];
        }
        let mut ret = Vec::new();
        for (i, &val) in v.iter().enumerate() {
            let lo = Self::helper(&v[..i]);
            let hi = Self::helper(&v[i + 1..]);
            for l in &lo {
                for r in &hi {
                    ret.push(Some(Rc::new(RefCell::new(TreeNode {
                        val,
                        left: l.clone(),
                        right: r.clone(),
                    }))))
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![
                to_tree(vec![Some(1), None, Some(2), None, Some(3)]),
                to_tree(vec![Some(1), None, Some(3), Some(2)]),
                to_tree(vec![Some(2), Some(1), Some(3)]),
                to_tree(vec![Some(3), Some(1), None, None, Some(2)]),
                to_tree(vec![Some(3), Some(2), None, Some(1)]),
            ],
            Solution::generate_trees(3)
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![to_tree(vec![Some(1)])], Solution::generate_trees(1))
    }
}
