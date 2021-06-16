pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut answer = Vec::new();
        let mut s = String::new();
        Self::backtrack(&mut answer, &mut s, 0, n);
        answer
    }
    fn backtrack(answer: &mut Vec<String>, s: &mut String, depth: usize, n: i32) {
        if depth == 0 && n == 0 {
            answer.push(s.clone());
            return;
        }
        if n > 0 {
            s.push('(');
            Self::backtrack(answer, s, depth + 1, n - 1);
            s.pop();
        }
        if depth > 0 {
            s.push(')');
            Self::backtrack(answer, s, depth - 1, n);
            s.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"],
            Solution::generate_parenthesis(3)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(vec!["()"], Solution::generate_parenthesis(1));
    }
}
