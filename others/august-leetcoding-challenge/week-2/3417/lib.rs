use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        hm.insert(0, 1);
        Solution::helper(&root, 0, sum, &mut hm)
    }
    fn helper(
        node: &Option<Rc<RefCell<TreeNode>>>,
        current: i32,
        target: i32,
        hm: &mut HashMap<i32, i32>,
    ) -> i32 {
        let mut ret = 0;
        if let Some(n) = node {
            let sum = current + n.borrow().val;
            if let Some(m) = hm.get(&(sum - target)) {
                ret += m;
            }
            *hm.entry(sum).or_default() += 1;
            ret += Solution::helper(&n.borrow().left, sum, target, hm);
            ret += Solution::helper(&n.borrow().right, sum, target, hm);
            *hm.entry(sum).or_default() -= 1;
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
            3,
            Solution::path_sum(
                to_tree(vec![
                    Some(10),
                    Some(5),
                    Some(-3),
                    Some(3),
                    Some(2),
                    None,
                    Some(11),
                    Some(3),
                    Some(-2),
                    None,
                    Some(1)
                ]),
                8
            )
        );
    }
}
