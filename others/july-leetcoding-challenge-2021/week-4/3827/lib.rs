use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&nums)
    }
    fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            None
        } else {
            let m = nums.len() / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[m],
                left: Self::helper(&nums[..m]),
                right: Self::helper(&nums[m + 1..]),
            })))
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
            to_tree(vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)]),
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![Some(3), Some(1)]),
            Solution::sorted_array_to_bst(vec![1, 3])
        );
    }
}
