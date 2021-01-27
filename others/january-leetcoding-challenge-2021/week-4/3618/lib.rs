pub struct Solution;

const DIV: u64 = 1_000_000_007;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut answer = 0_u64;
        for i in 1..=n as u64 {
            answer <<= 64 - i.leading_zeros();
            answer += i;
            answer %= DIV;
        }
        answer as i32
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
        assert_eq!(505379714, Solution::concatenated_binary(12));
    }
}
