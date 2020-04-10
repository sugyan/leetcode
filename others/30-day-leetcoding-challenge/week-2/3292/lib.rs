use std::collections::HashMap;

#[derive(Default)]
pub struct MinStack {
    v: Vec<i32>,
    hm: HashMap<i32, usize>,
    min: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        MinStack {
            min: std::i32::MAX,
            ..Default::default()
        }
    }

    pub fn push(&mut self, x: i32) {
        self.v.push(x);
        *self.hm.entry(x).or_insert(0) += 1;
        self.min = std::cmp::min(self.min, x);
    }

    pub fn pop(&mut self) {
        if let Some(x) = self.v.pop() {
            if let Some(n) = self.hm.get_mut(&x) {
                *n -= 1;
                if *n == 0 {
                    self.hm.remove(&x);
                    self.min = if let Some(min) = self.hm.keys().min() {
                        *min
                    } else {
                        std::i32::MAX
                    }
                }
            }
        }
    }

    pub fn top(&self) -> i32 {
        *self.v.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        self.min
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

    #[test]
    fn example_2() {
        // ["MinStack","push","push","push","top","pop","getMin","pop","getMin","pop","push","top","getMin","push","top","getMin","pop","getMin"]
        // [[],[2147483646],[2147483646],[2147483647],[],[],[],[],[],[],[2147483647],[],[],[-2147483648],[],[],[],[]]
        let mut obj = MinStack::new();
        obj.push(2_147_483_646);
        obj.push(2_147_483_646);
        obj.push(2_147_483_647);
        assert_eq!(2_147_483_647, obj.top());
        obj.pop();
        obj.push(-3);
        assert_eq!(-3, obj.get_min());
        obj.pop();
        assert_eq!(0, obj.top());
        assert_eq!(-2, obj.get_min());
    }
}
