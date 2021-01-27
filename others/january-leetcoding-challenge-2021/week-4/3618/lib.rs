pub struct Solution;

const DIV: i32 = 1_000_000_007;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut answer = 0;
        for i in 1..=n {
            for j in (0..32 - i.leading_zeros()).rev() {
                answer = ((answer << 1) + if i & 1 << j != 0 { 1 } else { 0 }) % DIV;
            }
        }
        answer
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
