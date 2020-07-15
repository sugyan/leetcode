pub struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut answer: String = String::new();
        let mut stack: Vec<char> = Vec::new();
        for c in ([" ", &s].concat()).chars().rev() {
            if c == ' ' {
                if !stack.is_empty() {
                    if !answer.is_empty() {
                        answer.push(' ');
                    }
                    while let Some(last) = stack.pop() {
                        answer.push(last);
                    }
                }
            } else {
                stack.push(c);
            }
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
