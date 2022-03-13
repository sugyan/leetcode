pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => stack.push(c),
                ')' if stack.last() == Some(&'(') => {
                    stack.pop();
                }
                '}' if stack.last() == Some(&'{') => {
                    stack.pop();
                }
                ']' if stack.last() == Some(&'[') => {
                    stack.pop();
                }
                _ => return false,
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn example_1() {
        assert_eq!(true, Solution::is_valid(String::from("()")));
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn example_2() {
        assert_eq!(true, Solution::is_valid(String::from("()[]{}")));
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn example_3() {
        assert_eq!(false, Solution::is_valid(String::from("(]")));
    }
}
