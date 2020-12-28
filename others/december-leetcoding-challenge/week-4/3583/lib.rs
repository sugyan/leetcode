pub struct Solution {}

impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let target = target.abs();
        for n in (-1.0 + (1.0 + 8.0 * target as f32).sqrt()) as i32 / 2.. {
            let m = n * (n + 1) / 2;
            if m >= target && (m - target) % 2 == 0 {
                return n;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::reach_number(3));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::reach_number(2));
    }
}
