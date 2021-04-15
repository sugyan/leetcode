pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        (0..n).fold((0, 1), |acc, _| (acc.1, acc.0 + acc.1)).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::fib(2));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::fib(3));
    }

    #[test]
    fn example_3() {
        assert_eq!(3, Solution::fib(4));
    }
}
