#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub struct NestedIterator {
    stack: Vec<NestedInteger>,
    next: Option<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut s = Self {
            stack: nested_list.into_iter().rev().collect::<Vec<_>>(),
            next: None,
        };
        s.advance_next();
        s
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        let ret = self.next.unwrap();
        self.advance_next();
        ret
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    fn advance_next(&mut self) {
        while let Some(last) = self.stack.pop() {
            match last {
                NestedInteger::Int(n) => {
                    self.next = Some(n);
                    return;
                }
                NestedInteger::List(list) => {
                    self.stack.extend(list.into_iter().rev());
                }
            }
        }
        self.next = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = NestedIterator::new(vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ]);
        assert_eq!(1, obj.next());
        assert_eq!(true, obj.has_next());
        assert_eq!(1, obj.next());
        assert_eq!(true, obj.has_next());
        assert_eq!(2, obj.next());
        assert_eq!(true, obj.has_next());
        assert_eq!(1, obj.next());
        assert_eq!(true, obj.has_next());
        assert_eq!(1, obj.next());
        assert_eq!(false, obj.has_next());
    }

    #[test]
    fn example_2() {
        let mut obj = NestedIterator::new(vec![NestedInteger::List(vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![NestedInteger::Int(6)]),
            ]),
        ])]);
        assert_eq!(1, obj.next());
        assert_eq!(true, obj.has_next());
        assert_eq!(4, obj.next());
        assert_eq!(true, obj.has_next());
        assert_eq!(6, obj.next());
        assert_eq!(false, obj.has_next());
    }
}
