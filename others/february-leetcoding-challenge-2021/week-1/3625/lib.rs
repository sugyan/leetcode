pub struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut n = n;
        n = (n & 0x5555_5555) + ((n & 0xAAAA_AAAA) >> 1);
        n = (n & 0x3333_3333) + ((n & 0xCCCC_CCCC) >> 2);
        n = (n & 0x0F0F_0F0F) + ((n & 0xF0F0_F0F0) >> 4);
        n = (n & 0x00FF_00FF) + ((n & 0xFF00_FF00) >> 8);
        n = (n & 0x0000_FFFF) + ((n & 0xFFFF_0000) >> 16);
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
            Solution::hammingWeight(0b0000_0000_0000_0000_0000_0000_0000_1011)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::hammingWeight(0b0000_0000_0000_0000_0000_0000_1000_0000)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            31,
            Solution::hammingWeight(0b1111_1111_1111_1111_1111_1111_1111_1101)
        );
    }
}
