pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }
        let v: Vec<u8> = s.bytes().collect();
        let mut j;
        let (mut l, mut r) = (0, 0);
        for i in 0..v.len() {
            {
                j = 0;
                while i >= j && i + 1 + j < v.len() && v[i - j] == v[i + 1 + j] {
                    j += 1;
                }
                if j * 2 > r - l {
                    l = i + 1 - j;
                    r = i + j + 1;
                }
            }
            {
                j = 0;
                while i >= j && i + j < v.len() && v[i - j] == v[i + j] {
                    j += 1;
                }
                if j * 2 - 1 > r - l {
                    l = i + 1 - j;
                    r = i + j;
                }
            }
        }
        s[l..r].to_string()
    }
}

fn main() {
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn example_1() {
        let mut answers = HashSet::new();
        answers.insert("bab".to_string());
        answers.insert("aba".to_string());
        assert!(answers.contains(&Solution::longest_palindrome("babad".to_string())));
    }

    #[test]
    fn example_2() {
        assert_eq!("bb", Solution::longest_palindrome("cbbd".to_string()));
    }
}
