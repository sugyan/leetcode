pub struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        (31 + num.count_ones()).saturating_sub(num.leading_zeros()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(6, Solution::number_of_steps(14));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::number_of_steps(8));
    }

    #[test]
    fn example_3() {
        assert_eq!(12, Solution::number_of_steps(123));
    }
}
