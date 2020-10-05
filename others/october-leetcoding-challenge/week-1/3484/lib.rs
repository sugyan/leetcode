pub struct Solution {}

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let mut m = 1;
        while m < n {
            m = (m << 1) + 1
        }
        m - n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::bitwise_complement(5));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::bitwise_complement(7));
    }

    #[test]
    fn example_3() {
        assert_eq!(5, Solution::bitwise_complement(10));
    }
}
