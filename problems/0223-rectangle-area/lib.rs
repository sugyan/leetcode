pub struct Solution {}

impl Solution {
    #[allow(
        clippy::many_single_char_names,
        clippy::too_many_arguments,
        clippy::unnecessary_unwrap
    )]
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        ((a - c) * (b - d)).abs() + ((e - g) * (f - h)).abs()
            - if a >= g || c <= e || b >= h || d <= f {
                0
            } else {
                (std::cmp::min(c, g) - std::cmp::max(a, e))
                    * (std::cmp::min(d, h) - std::cmp::max(b, f))
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(45, Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2));
    }
}
