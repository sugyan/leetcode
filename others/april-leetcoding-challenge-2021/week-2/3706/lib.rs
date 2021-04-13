#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub struct NestedIterator {
    v: Vec<i32>,
    i: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut v = Vec::new();
        for nested_integer in nested_list.into_iter() {
            match nested_integer {
                NestedInteger::Int(n) => v.push(n),
                NestedInteger::List(nested_list) => {
                    let mut iter = Self::new(nested_list);
                    while iter.has_next() {
                        v.push(iter.next())
                    }
                }
            }
        }
        Self { v, i: 0 }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        let ret = self.v[self.i];
        self.i += 1;
        ret
    }

    pub fn has_next(&self) -> bool {
        self.i < self.v.len()
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
