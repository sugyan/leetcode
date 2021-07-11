#[derive(Default)]
pub struct MedianFinder {
    v: Vec<i32>,
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
        self.v.insert(
            match self.v.binary_search(&num) {
                Ok(i) => i,
                Err(i) => i,
            },
            num,
        );
    }

    pub fn find_median(&self) -> f64 {
        if self.v.len() % 2 != 0 {
            self.v[self.v.len() / 2] as f64
        } else {
            (self.v[self.v.len() / 2] + self.v[self.v.len() / 2 - 1]) as f64 / 2.0
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
