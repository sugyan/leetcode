pub struct Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let s = s.as_bytes();
        let mut v = vec![0; s.len() + 1];
        for i in 0..s.len() {
            v[i + 1] = v[i] + if s[i] == b'1' { 1 } else { 0 };
        }
        (0..v.len())
            .map(|i| v[i] + s.len() - i - (v[s.len()] - v[i]))
            .min()
            .unwrap() as i32
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
