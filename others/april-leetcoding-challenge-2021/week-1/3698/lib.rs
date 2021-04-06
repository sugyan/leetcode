pub struct Solution;

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        (n / 2) * (n / 2) + if n % 2 == 1 { n / 2 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::min_operations(3));
    }

    #[test]
    fn example_2() {
        assert_eq!(9, Solution::min_operations(6));
    }
}
