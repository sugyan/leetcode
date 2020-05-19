#[derive(Default)]
pub struct StockSpanner {
    s: Vec<(i32, usize)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn next(&mut self, price: i32) -> i32 {
        let mut ret = 1;
        while let Some(last) = self.s.last() {
            if last.0 > price {
                break;
            }
            ret += last.1;
            self.s.pop();
        }
        self.s.push((price, ret));
        ret as i32
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = StockSpanner::new();
        assert_eq!(1, obj.next(100));
        assert_eq!(1, obj.next(80));
        assert_eq!(1, obj.next(60));
        assert_eq!(2, obj.next(70));
        assert_eq!(1, obj.next(60));
        assert_eq!(4, obj.next(75));
        assert_eq!(6, obj.next(85));
    }
}
