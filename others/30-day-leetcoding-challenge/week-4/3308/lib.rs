pub struct Solution {}

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut answer = 0;
        let mut mask = 0x4000_0000;
        for _ in 0..31 {
            if (m & mask) != (n & mask) {
                break;
            }
            answer += m & mask;
            mask >>= 1;
        }
        answer
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
        assert_eq!(0, Solution::range_bitwise_and(0, 1));
    }
}
