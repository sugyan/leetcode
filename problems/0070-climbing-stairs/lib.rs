pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (1..n).fold((1, 1), |x, _| (x.1, x.0 + x.1)).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::climb_stairs(2));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::climb_stairs(3));
    }
}
