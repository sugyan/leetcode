pub struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut answer, mut depth) = (0, 0);
        for (i, b) in s.iter().enumerate() {
            match b {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if s[i - 1] == b'(' {
                        answer += 1 << depth;
                    }
                }
                _ => unreachable!(),
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
}
