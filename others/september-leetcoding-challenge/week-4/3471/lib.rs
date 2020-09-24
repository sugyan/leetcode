pub struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut d = [0; 26];
        for c in t.as_bytes() {
            d[(c - b'a') as usize] += 1;
        }
        for c in s.as_bytes() {
            d[(c - b'a') as usize] -= 1;
        }
        (b'a' + (0..26).find(|&i| d[i] == 1).unwrap() as u8) as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            'e',
            Solution::find_the_difference(String::from("abcd"), String::from("abcde"))
        );
    }
}
