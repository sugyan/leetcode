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
        assert!((165.0 - Solution::angle_clock(12, 30)).abs() < f64::EPSILON);
    }

    #[test]
    fn example_2() {
        assert!((75.0 - Solution::angle_clock(3, 30)).abs() < f64::EPSILON);
    }

    #[test]
    fn example_3() {
        assert!((7.5 - Solution::angle_clock(3, 15)).abs() < f64::EPSILON);
    }

    #[test]
    fn example_4() {
        assert!((155.0 - Solution::angle_clock(4, 50)).abs() < f64::EPSILON);
    }

    #[test]
    fn example_5() {
        assert!((0.0 - Solution::angle_clock(12, 0)).abs() < f64::EPSILON);
    }
}
