pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut answer = vec![];
        Solution::gen(&mut answer, "".to_string(), n, n);
        answer
    }

    fn gen(v: &mut Vec<String>, s: String, l: i32, r: i32) {
        if l == 0 && r == 0 {
            v.push(s);
            return;
        }
        if l > 0 {
            Solution::gen(v, s.clone() + "(", l - 1, r);
        }
        if l < r {
            Solution::gen(v, s + ")", l, r - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::generate_parenthesis(3);
        ret.sort();
        assert_eq!(vec!["((()))", "(()())", "(())()", "()(())", "()()()"], ret);
    }
}
