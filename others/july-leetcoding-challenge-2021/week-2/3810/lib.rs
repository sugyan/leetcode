use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
pub struct MedianFinder {
    small: BinaryHeap<i32>,
    large: BinaryHeap<Reverse<i32>>,
    odd: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_num(&mut self, num: i32) {
        if self.odd {
            self.small.push(num);
            if let Some(max) = self.small.pop() {
                self.large.push(Reverse(max));
            }
        } else {
            self.large.push(Reverse(num));
            if let Some(Reverse(min)) = self.large.pop() {
                self.small.push(min);
            }
        }
        self.odd = !self.odd;
    }

    pub fn find_median(&self) -> f64 {
        if self.odd {
            *self.small.peek().unwrap() as f64
        } else {
            (self.small.peek().unwrap() + self.large.peek().unwrap().0) as f64 / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = MedianFinder::new();
        obj.add_num(1);
        obj.add_num(2);
        assert!((1.5 - obj.find_median()).abs() < std::f64::EPSILON);
        obj.add_num(3);
        assert!((2.0 - obj.find_median()).abs() < std::f64::EPSILON);
    }
}
