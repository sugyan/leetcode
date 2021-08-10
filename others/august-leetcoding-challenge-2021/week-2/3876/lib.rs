pub struct Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut v = vec![0; s.len() + 1];
        {
            let mut c0 = 0;
            for (i, &u) in s.as_bytes().iter().enumerate() {
                if u == b'1' {
                    c0 += 1;
                }
                v[i + 1] += c0;
            }
        }
        {
            let mut c1 = 0;
            for (i, &u) in s.as_bytes().iter().enumerate().rev() {
                if u == b'0' {
                    c1 += 1;
                }
                v[i] += c1;
            }
        }
        *v.iter().min().unwrap()
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
