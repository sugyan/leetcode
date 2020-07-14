pub struct Solution {}

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let angle = (minutes as f64 * 6.0 - (hour as f64 + minutes as f64 / 60.0) * 30.0).abs();
        angle.min(360.0 - angle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(165.0, Solution::angle_clock(12, 30));
    }

    #[test]
    fn example_2() {
        assert_eq!(75.0, Solution::angle_clock(3, 30));
    }

    #[test]
    fn example_3() {
        assert_eq!(7.5, Solution::angle_clock(3, 15));
    }

    #[test]
    fn example_4() {
        assert_eq!(155.0, Solution::angle_clock(4, 50));
    }

    #[test]
    fn example_5() {
        assert_eq!(0.0, Solution::angle_clock(12, 0));
    }
}
