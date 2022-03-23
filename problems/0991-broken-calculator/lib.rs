pub struct Solution;

impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        let mut answer = 0;
        let mut n = target;
        while n > start_value {
            n = if n & 1 == 0 { n >> 1 } else { n + 1 };
            answer += 1;
        }
        answer += start_value - n;
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::broken_calc(2, 3));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::broken_calc(5, 8));
    }

    #[test]
    fn example_3() {
        assert_eq!(3, Solution::broken_calc(3, 10));
    }
}
