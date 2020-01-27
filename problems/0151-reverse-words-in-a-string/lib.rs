pub struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut v: Vec<char> = Vec::new();
        let mut flg = false;
        let mut answer: String = String::new();
        for c in s.chars().rev().chain(" ".chars()) {
            match c {
                ' ' => {
                    if flg {
                        if !answer.is_empty() {
                            answer.push(' ');
                        }
                        for c in v.iter().rev() {
                            answer.push(*c);
                        }
                        v.clear();
                    }
                }
                c => {
                    v.push(c);
                }
            }
            flg = c != ' ';
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "blue is sky the",
            Solution::reverse_words("the sky is blue".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "world! hello",
            Solution::reverse_words("  hello world!  ".to_string())
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "example good a",
            Solution::reverse_words("a good   example".to_string())
        );
    }
}
