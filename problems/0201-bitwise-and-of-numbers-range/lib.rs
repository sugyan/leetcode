pub struct Solution {}

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut n = n;
        while n > m {
            n &= n - 1;
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::range_bitwise_and(5, 7));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::range_bitwise_and(0, 0));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::range_bitwise_and(1, 2147483647));
    }
}
