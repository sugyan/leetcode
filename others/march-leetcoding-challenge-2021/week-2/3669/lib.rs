pub struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let mut v = vec![false; 1 << k];
        let mut n = 0;
        let mask = (1 << k) - 1;
        for (i, &b) in s.as_bytes().iter().enumerate() {
            n <<= 1;
            n += if b == b'1' { 1 } else { 0 };
            n &= mask;
            if i + 1 < k {
                continue;
            }
            v[n] = true;
        }
        v.iter().all(|&b| b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::has_all_codes(String::from("00110110"), 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(true, Solution::has_all_codes(String::from("00110"), 2));
    }

    #[test]
    fn example_3() {
        assert_eq!(true, Solution::has_all_codes(String::from("0110"), 1));
    }

    #[test]
    fn example_4() {
        assert_eq!(false, Solution::has_all_codes(String::from("0110"), 2));
    }

    #[test]
    fn example_5() {
        assert_eq!(
            false,
            Solution::has_all_codes(String::from("0000000001011100"), 4)
        );
    }
}
