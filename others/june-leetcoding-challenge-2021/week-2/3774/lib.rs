#[derive(Default)]
pub struct MyCalendar {
    events: Vec<(i32, i32)>,
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
        if let Err(i) = self.events.binary_search_by_key(&start, |&(s, _)| s) {
            if (i == 0 || self.events[i - 1].1 <= start)
                && (i == self.events.len() || end <= self.events[i].0)
            {
                self.events.insert(i, (start, end));
                return true;
            }
        }
        false
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
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
