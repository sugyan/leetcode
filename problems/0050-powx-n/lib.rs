pub struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut answer = 1.0;
        let mut x = if n < 0 { 1.0 / x } else { x };
        let mut n = (n as i64).abs();
        while n != 0 {
            if n & 1 != 0 {
                answer *= x;
            }
            x *= x;
            n >>= 1;
        }
        return answer;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!((1024.00000 - Solution::my_pow(2.00000, 10)).abs() < 0.00001);
    }

    #[test]
    fn example_2() {
        assert!((9.26100 - Solution::my_pow(2.10000, 3)).abs() <= 0.00001);
    }

    #[test]
    fn example_3() {
        assert!((0.25000 - Solution::my_pow(2.00000, -2)).abs() <= 0.00001);
    }
}
