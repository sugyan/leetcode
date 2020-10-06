use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(match root {
            None => Rc::new(RefCell::new(TreeNode::new(val))),
            Some(r) => {
                if r.borrow().val > val {
                    let node = Solution::insert_into_bst(r.borrow().left.clone(), val);
                    r.borrow_mut().left = node;
                } else {
                    let node = Solution::insert_into_bst(r.borrow().right.clone(), val);
                    r.borrow_mut().right = node
                }
                r
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        assert_eq!(
            to_tree(vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)]),
            Solution::insert_into_bst(
                to_tree(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]),
                5
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_tree(vec![
                Some(40),
                Some(20),
                Some(60),
                Some(10),
                Some(30),
                Some(50),
                Some(70),
                None,
                None,
                Some(25)
            ]),
            Solution::insert_into_bst(
                to_tree(vec![
                    Some(40),
                    Some(20),
                    Some(60),
                    Some(10),
                    Some(30),
                    Some(50),
                    Some(70)
                ]),
                25
            )
        );
    }
}
