pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![0];
        let mut answer = 0;
        for (i, &b) in s.as_bytes().iter().enumerate() {
            match b {
                b'(' => {
                    stack.push(i + 1);
                }
                b')' => {
                    stack.pop();
                    if stack.is_empty() {
                        stack.push(i + 1);
                    } else if let Some(&last) = stack.last() {
                        answer = answer.max(i - last + 1);
                    }
                }
                _ => unreachable!(),
            };
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::longest_valid_parentheses(String::from("(()")));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            4,
            Solution::longest_valid_parentheses(String::from(")()())"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::longest_valid_parentheses(String::from("")));
    }
}
