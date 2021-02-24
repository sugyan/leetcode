pub struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut v = vec![0; 1];
        let mut i = 1;
        for c in s.chars() {
            match c {
                '(' => {
                    if i >= v.len() {
                        v.push(0);
                    } else {
                        v[i] = 0;
                    }
                    i += 1;
                }
                ')' => {
                    i -= 1;
                    v[i - 1] += if v[i] == 0 { 1 } else { v[i] * 2 };
                }
                _ => unreachable!(),
            }
        }
        v[0]
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
