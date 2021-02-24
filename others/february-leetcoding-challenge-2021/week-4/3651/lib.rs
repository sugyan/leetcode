pub struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![0; 1];
        for c in s.chars() {
            match c {
                '(' => stack.push(0),
                ')' => {
                    if let Some(popped) = stack.pop() {
                        if let Some(last) = stack.last_mut() {
                            *last += std::cmp::max(popped * 2, 1);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        stack[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::score_of_parentheses(String::from("()")));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::score_of_parentheses(String::from("(())")));
    }

    #[test]
    fn example_3() {
        assert_eq!(2, Solution::score_of_parentheses(String::from("()()")));
    }

    #[test]
    fn example_4() {
        assert_eq!(6, Solution::score_of_parentheses(String::from("(()(()))")));
    }
}
