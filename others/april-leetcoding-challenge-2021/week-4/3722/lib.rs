pub struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 1_162_261_467 % n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::is_power_of_three(27));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::is_power_of_three(0));
    }

    #[test]
    fn example_3() {
        assert_eq!(true, Solution::is_power_of_three(9));
    }

    #[test]
    fn example_4() {
        assert_eq!(false, Solution::is_power_of_three(45));
    }
}
