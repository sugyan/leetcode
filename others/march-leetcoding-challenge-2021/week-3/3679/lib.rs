use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let digit_counts = |n: i32| -> [usize; 10] {
            let mut d = [0; 10];
            for &u in n.to_string().as_bytes() {
                d[(u - b'0') as usize] += 1;
            }
            d
        };
        let hs = (0..)
            .map(|i| 1 << i)
            .take_while(|&n| n <= 10_i32.pow(9))
            .map(digit_counts)
            .collect::<HashSet<_>>();
        hs.contains(&digit_counts(n))
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
