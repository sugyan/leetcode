pub struct Solution;

const DIV: u64 = 1_000_000_007;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        (2..=n).fold(1_u64, |acc, x| {
            ((acc << (32 - x.leading_zeros())) + x as u64) % DIV
        }) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::concatenated_binary(1));
    }

    #[test]
    fn example_2() {
        assert_eq!(27, Solution::concatenated_binary(3));
    }

    #[test]
    fn example_3() {
        assert_eq!(505_379_714, Solution::concatenated_binary(12));
    }
}
