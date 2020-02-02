use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct BSTIterator {
    stack: Rc<RefCell<Vec<Rc<RefCell<TreeNode>>>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut node: Option<Rc<RefCell<TreeNode>>> = root;
        while node.is_some() {
            while let Some(n) = node {
                stack.push(n.clone());
                node = n.borrow().left.clone();
            }
        }
        BSTIterator {
            stack: Rc::new(RefCell::new(stack)),
        }
    }

    /** @return the next smallest number */
    pub fn next(&self) -> i32 {
        let last = self.stack.borrow_mut().pop().unwrap();
        let val = last.borrow().val;
        let mut node: Option<Rc<RefCell<TreeNode>>> = last.borrow().right.clone();
        while let Some(n) = node {
            self.stack.borrow_mut().push(n.clone());
            node = n.borrow().left.clone();
        }
        val
    }

    /** @return whether we have a next smallest number */
    pub fn has_next(&self) -> bool {
        !self.stack.borrow().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::to_tree;

    #[test]
    fn example_1() {
        let root = to_tree(vec![
            Some(7),
            Some(3),
            Some(15),
            None,
            None,
            Some(9),
            Some(20),
        ]);
        let obj = BSTIterator::new(root);
        assert_eq!(3, obj.next());
        assert_eq!(7, obj.next());
        assert_eq!(true, obj.has_next());
        assert_eq!(9, obj.next());
        assert_eq!(true, obj.has_next());
        assert_eq!(15, obj.next());
        assert_eq!(true, obj.has_next());
        assert_eq!(20, obj.next());
        assert_eq!(false, obj.has_next());
    }
}
