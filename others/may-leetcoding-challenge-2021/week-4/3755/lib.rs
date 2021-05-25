pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        tokens.iter().fold(Vec::new(), |mut stack, x| {
            if let Ok(x) = x.parse() {
                stack.push(x);
            } else if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
                stack.push(match x.as_str() {
                    "+" => lhs + rhs,
                    "-" => lhs - rhs,
                    "*" => lhs * rhs,
                    "/" => lhs / rhs,
                    _ => unreachable!(),
                })
            }
            stack
        })[0]
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
                    .into_iter()
                    .map(String::from)
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
                    .into_iter()
                    .map(String::from)
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
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }
}
