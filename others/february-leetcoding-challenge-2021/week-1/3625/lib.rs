pub struct Solution;

impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut n = n;
        n = (n & 0x5555_5555) + (n >> 1 & 0x5555_5555);
        n = (n & 0x3333_3333) + (n >> 2 & 0x3333_3333);
        n = (n & 0x0F0F_0F0F) + (n >> 4 & 0x0F0F_0F0F);
        n = (n & 0x00FF_00FF) + (n >> 8 & 0x00FF_00FF);
        n = (n & 0x0000_FFFF) + (n >> 16 & 0x0000_FFFF);
        n as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::hamming_weight(0b0000_0000_0000_0000_0000_0000_0000_1011)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::hamming_weight(0b0000_0000_0000_0000_0000_0000_1000_0000)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            31,
            Solution::hamming_weight(0b1111_1111_1111_1111_1111_1111_1111_1101)
        );
    }
}
