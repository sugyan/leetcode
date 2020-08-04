pub struct Solution {}

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        num & (num - 1) == 0 && (num - 1) % 3 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::is_power_of_four(16));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::is_power_of_four(5));
    }
}
