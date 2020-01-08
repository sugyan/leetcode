use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        if let Some(r) = root {
            let val = r.borrow().val;
            if val == sum && r.borrow().left.is_none() && r.borrow().right.is_none() {
                return vec![vec![val]];
            }
            for ret in Solution::path_sum(r.borrow().left.clone(), sum - val).iter_mut() {
                ret.insert(0, val);
                answer.push(ret.to_vec());
            }
            for ret in Solution::path_sum(r.borrow().right.clone(), sum - val).iter_mut() {
                ret.insert(0, val);
                answer.push(ret.to_vec());
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        let mut ret = Solution::path_sum(
            to_tree(vec![
                Some(5),
                Some(4),
                Some(8),
                Some(11),
                None,
                Some(13),
                Some(4),
                Some(7),
                Some(2),
                None,
                None,
                Some(5),
                Some(1),
            ]),
            22,
        );
        ret.sort();
        assert_eq!(vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]], ret);
    }
}
