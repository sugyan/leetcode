pub struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        for token in tokens.iter() {
            if let Ok(n) = token.parse() {
                stack.push(n);
            } else {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match token.as_str() {
                    "+" => stack.push(lhs + rhs),
                    "-" => stack.push(lhs - rhs),
                    "*" => stack.push(lhs * rhs),
                    "/" => stack.push(lhs / rhs),
                    _ => {}
                }
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
        assert_eq!(
            9,
            Solution::eval_rpn(
                vec!["2", "1", "+", "3", "*"]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            6,
            Solution::eval_rpn(
                vec!["4", "13", "5", "/", "+"]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect()
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            22,
            Solution::eval_rpn(
                vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect()
            )
        );
    }
}
