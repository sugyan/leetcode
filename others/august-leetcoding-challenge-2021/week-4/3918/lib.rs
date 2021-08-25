pub struct Solution;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        for a in 0..=(c as f64).sqrt() as i32 {
            let b = ((c - a * a) as f64).sqrt() as i32;
            if a * a + b * b == c {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::judge_square_sum(5));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::judge_square_sum(3));
    }

    #[test]
    fn example_3() {
        assert_eq!(true, Solution::judge_square_sum(4));
    }

    #[test]
    fn example_4() {
        assert_eq!(true, Solution::judge_square_sum(2));
    }

    #[test]
    fn example_5() {
        assert_eq!(true, Solution::judge_square_sum(1));
    }
}
