pub struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let (mut l, mut r) = (0, num as i64);
        while l < r {
            let m = l + (r - l) / 2;
            if m * m < num as i64 {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l * l == num as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::is_perfect_square(16));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::is_perfect_square(14));
    }
}
