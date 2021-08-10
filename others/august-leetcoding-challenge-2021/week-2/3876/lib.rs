pub struct Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let (mut ones, mut flip) = (0, 0);
        for c in s.chars() {
            match c {
                '0' => flip = ones.min(flip + 1),
                '1' => ones += 1,
                _ => unreachable!(),
            }
        }
        flip
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::min_flips_mono_incr(String::from("00110")));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::min_flips_mono_incr(String::from("010110")));
    }

    #[test]
    fn example_3() {
        assert_eq!(2, Solution::min_flips_mono_incr(String::from("00011000")));
    }
}
