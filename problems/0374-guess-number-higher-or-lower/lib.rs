use std::cmp::Ordering;

pub struct Solution;

static mut NUMBER: i32 = 1;

unsafe fn guess(num: i32) -> i32 {
    match NUMBER.cmp(&num) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

impl Solution {
    /// # Safety
    /// ?
    pub unsafe fn guess_number(n: i32) -> i32 {
        let (mut lo, mut hi) = (1, n);
        while lo < hi {
            let m = lo + (hi - lo) / 2;
            match guess(m) {
                -1 => hi = m - 1,
                1 => lo = m + 1,
                _ => return m,
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        unsafe {
            NUMBER = 6;
            assert_eq!(6, Solution::guess_number(10));
        }
    }

    #[test]
    fn example_2() {
        unsafe {
            NUMBER = 1;
            assert_eq!(1, Solution::guess_number(1));
        }
    }

    #[test]
    fn example_3() {
        unsafe {
            NUMBER = 1;
            assert_eq!(1, Solution::guess_number(2));
        }
    }

    #[test]
    fn example_4() {
        unsafe {
            NUMBER = 2;
            assert_eq!(2, Solution::guess_number(2));
        }
    }
}
