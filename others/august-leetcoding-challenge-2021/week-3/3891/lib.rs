pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let target = t
            .as_bytes()
            .iter()
            .fold(vec![0; 128], |mut acc, &u| {
                acc[u as usize] += 1;
                acc
            })
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if c > 0 { Some((i, c)) } else { None })
            .collect::<Vec<_>>();
        let s = s.as_bytes();

        let mut count = vec![0; 128];
        let (mut lo, mut hi) = (0, 0);
        let mut min = (0, s.len() + 1);
        while hi < s.len() {
            while hi < s.len() && target.iter().any(|&(i, c)| count[i] < c) {
                count[s[hi] as usize] += 1;
                hi += 1;
            }
            while lo < hi && target.iter().all(|&(i, c)| count[i] >= c) {
                if hi - lo < min.1 - min.0 {
                    min = (lo, hi);
                }
                count[s[lo] as usize] -= 1;
                lo += 1;
            }
        }
        if min.1 > s.len() {
            String::new()
        } else {
            s[min.0..min.1]
                .iter()
                .map(|&u| u as char)
                .collect::<String>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "BANC",
            Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABC"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "a",
            Solution::min_window(String::from("a"), String::from("a"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "",
            Solution::min_window(String::from("a"), String::from("aa"))
        );
    }
}
