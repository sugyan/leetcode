pub struct Solution;

impl Solution {
    pub fn broken_calc(x: i32, y: i32) -> i32 {
        let mut y = y;
        let mut answer = 0;
        while y > x {
            if y & 1 == 0 {
                y >>= 1;
            } else {
                y += 1;
            }
            answer += 1;
        }
        answer + x - y
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

    #[test]
    fn example_4() {
        assert_eq!(1023, Solution::broken_calc(1024, 1));
    }
}
