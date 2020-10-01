use std::collections::VecDeque;

#[derive(Default)]
pub struct RecentCounter {
    vd: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        while let Some(&front) = self.vd.front() {
            if front + 3000 >= t {
                break;
            }
            self.vd.pop_front();
        }
        self.vd.push_back(t);
        self.vd.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = RecentCounter::new();
        assert_eq!(1, obj.ping(1));
        assert_eq!(2, obj.ping(100));
        assert_eq!(3, obj.ping(3001));
        assert_eq!(3, obj.ping(3002));
    }
}
