pub struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut answer = 0;
        for c in s.as_bytes() {
            answer ^= c;
        }
        for c in t.as_bytes() {
            answer ^= c;
        }
        answer as char
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
