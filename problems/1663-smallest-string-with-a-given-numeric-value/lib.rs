pub struct Solution;

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut v = vec![0; n as usize];
        let mut k = k - n;
        for e in v.iter_mut().rev() {
            let m = k.min(25);
            *e = b'a' + m as u8;
            k -= m;
        }
        v.iter().map(|&u| u as char).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("aay", Solution::get_smallest_string(3, 27));
    }

    #[test]
    fn example_2() {
        assert_eq!("aaszz", Solution::get_smallest_string(5, 73));
    }
}
