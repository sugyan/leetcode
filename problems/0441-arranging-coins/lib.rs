pub struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let (mut lo, mut hi) = (0, n as i64);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if n as i64 >= (mid + 1) * (mid + 2) / 2 {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::arrange_coins(5));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::arrange_coins(8));
    }
}
