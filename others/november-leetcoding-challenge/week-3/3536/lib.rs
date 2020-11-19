pub struct Solution {}

impl Solution {
    pub fn decode_string(s: String) -> String {
        Solution::helper(&s)
    }
    fn helper(s: &str) -> String {
        let mut ret = String::new();
        let mut m = 0;
        let mut l = 0;
        let mut depth = 0;
        for (i, &b) in s.as_bytes().iter().enumerate() {
            match b {
                b'[' => {
                    if depth == 0 {
                        l = i + 1;
                    }
                    depth += 1;
                }
                b']' => {
                    depth -= 1;
                    if depth == 0 {
                        ret += Solution::helper(&s[l..i]).repeat(m).as_str();
                        m = 0;
                    }
                }
                b'0'..=b'9' => {
                    if depth == 0 {
                        m = m * 10 + (b - b'0') as usize;
                    }
                }
                b => {
                    if depth == 0 {
                        ret.push(b as char);
                    }
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "aaabcbc",
            Solution::decode_string(String::from("3[a]2[bc]"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "accaccacc",
            Solution::decode_string(String::from("3[a2[c]]"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "abcabccdcdcdef",
            Solution::decode_string(String::from("2[abc]3[cd]ef"))
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            "abccdcdcdxyz",
            Solution::decode_string(String::from("abc3[cd]xyz"))
        );
    }
}
