pub struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        (0..n).fold([0, 1, 1], |v, _| [v[1], v[2], v[0] + v[1] + v[2]])[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::tribonacci(4));
    }

    #[test]
    fn example_2() {
        assert_eq!(1389537, Solution::tribonacci(25));
    }
}
