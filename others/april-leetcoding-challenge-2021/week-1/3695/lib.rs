pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut v = Vec::new();
        let mut answer = 0;
        let mut depth = 0;
        for (i, &b) in s.as_bytes().iter().enumerate() {
            match b {
                b'(' => {
                    if depth >= v.len() {
                        v.push(i);
                    }
                    depth += 1;
                }
                b')' => {
                    if depth > 0 {
                        depth -= 1;
                        answer = answer.max(i - v[depth] + 1);
                        if depth < v.len() - 1 {
                            v.pop();
                        }
                    } else {
                        v.clear();
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
