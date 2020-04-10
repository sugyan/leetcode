#[derive(Default)]
pub struct MinStack {
    v: Vec<i32>,
    m: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, x: i32) {
        self.v.push(x);
        if let Some(last) = self.m.last() {
            if x > *last {
                return;
            }
        }
        self.m.push(x);
    }

    pub fn pop(&mut self) {
        if let Some(last) = self.v.pop() {
            if last == self.get_min() {
                self.m.pop();
            }
        }
    }

    pub fn top(&self) -> i32 {
        *self.v.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.m.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = MinStack::new();
        obj.push(-2);
        obj.push(0);
        obj.push(-3);
        assert_eq!(-3, obj.get_min());
        obj.pop();
        assert_eq!(0, obj.top());
        assert_eq!(-2, obj.get_min());
    }
}
