#[derive(Default)]
pub struct MinStack {
    stack: Vec<i32>,
    min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        if let Some(&min) = self.min.last() {
            if val > min {
                return;
            }
        }
        self.min.push(val);
    }

    pub fn pop(&mut self) {
        if let Some(last) = self.stack.pop() {
            if last == self.get_min() {
                self.min.pop();
            }
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
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
