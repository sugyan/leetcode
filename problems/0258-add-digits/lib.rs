pub struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num < 10 {
            num
        } else {
            Self::add_digits(num.to_string().bytes().fold(0, |acc, x| acc + x - b'0') as i32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::add_digits(38));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::add_digits(0));
    }
}
