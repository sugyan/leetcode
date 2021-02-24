pub struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut answer = 0;
        let (mut count, mut prev) = (0, None);
        for c in s.chars() {
            match c {
                '(' => count += 1,
                ')' => {
                    count -= 1;
                    if prev == Some('(') {
                        answer += 1 << count;
                    }
                }
                _ => unreachable!(),
            }
            prev = Some(c);
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

    #[test]
    fn example_4() {
        assert_eq!(6, Solution::score_of_parentheses(String::from("(()(()))")));
    }
}
