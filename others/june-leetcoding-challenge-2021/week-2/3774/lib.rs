use std::collections::BTreeMap;

#[derive(Default)]
pub struct MyCalendar {
    events: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some(prev) = self.events.range(..start).last() {
            if *prev.1 > start {
                return false;
            }
        }
        if let Some(next) = self.events.range(start..).next() {
            if *next.0 < end {
                return false;
            }
        }
        self.events.insert(start, end);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = MyCalendar::new();
        assert_eq!(true, obj.book(10, 20));
        assert_eq!(false, obj.book(15, 25));
        assert_eq!(true, obj.book(20, 30));
    }
}
