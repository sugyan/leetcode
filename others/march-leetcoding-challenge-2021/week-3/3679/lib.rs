pub struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let digit_counts = |n: i32| -> [usize; 10] {
            let mut n = n;
            let mut d = [0; 10];
            while n > 0 {
                d[(n % 10) as usize] += 1;
                n /= 10;
            }
            d
        };
        let counts = digit_counts(n);
        (0..31).any(|i| digit_counts(1 << i) == counts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::reordered_power_of2(1));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::reordered_power_of2(10));
    }

    #[test]
    fn example_3() {
        assert_eq!(true, Solution::reordered_power_of2(16));
    }

    #[test]
    fn example_4() {
        assert_eq!(false, Solution::reordered_power_of2(24));
    }

    #[test]
    fn example_5() {
        assert_eq!(true, Solution::reordered_power_of2(46));
    }
}
