pub struct Solution {}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<(usize, String)> = Vec::new();
        let (mut n, mut str) = (0, String::new());
        for c in s.chars() {
            match c {
                '[' => {
                    stack.push((n, str.clone()));
                    n = 0;
                    str.clear();
                }
                ']' => {
                    if let Some(last) = stack.pop() {
                        str = last.1 + str.repeat(last.0).as_str();
                    }
                }
                '0'..='9' => n = n * 10 + (c as u8 - b'0') as usize,
                c => str.push(c),
            }
        }
        str
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
