use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }
        return Solution::generate((1..=n).collect::<Vec<i32>>());
    }
    fn generate(v: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if v.is_empty() {
            return vec![None];
        }
        let mut answer: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        for i in 0..v.len() {
            let l = Solution::generate(Vec::from(v.get(0..i).unwrap()));
            let r = Solution::generate(Vec::from(v.get(i + 1..).unwrap()));
            for j in l.iter() {
                for k in r.iter() {
                    let node = Rc::new(RefCell::new(TreeNode::new(v[i])));
                    node.borrow_mut().left = j.clone();
                    node.borrow_mut().right = k.clone();
                    answer.push(Some(node));
                }
            }
        }
        return answer;
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
                to_tree(vec![Some(3), Some(2), None, Some(1)])
            ],
            Solution::generate_trees(3)
        );
    }
}
