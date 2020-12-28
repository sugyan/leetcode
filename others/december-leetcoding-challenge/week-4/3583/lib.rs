pub struct Solution {}

impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let target = if target < 0 { -target } else { target };
        for n in std::cmp::max((target as f32 * 2.0).sqrt() as i32, 1) - 1.. {
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
