pub struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut answer: Vec<Vec<String>> = Vec::new();
        let mut v: Vec<String> = Vec::new();
        let s: Vec<char> = s.chars().collect();
        Solution::helper(&mut answer, &mut v, &s);
        answer
    }
    fn helper(answer: &mut Vec<Vec<String>>, v: &mut Vec<String>, s: &[char]) {
        if s.is_empty() {
            answer.push(v.clone());
            return;
        }
        for i in 1..=s.len() {
            if (0..i / 2).all(|j| s[j] == s[i - 1 - j]) {
                v.push(s[0..i].iter().collect());
                Solution::helper(answer, v, &s[i..]);
                v.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::partition("aab".to_string());
        ret.sort();
        assert_eq!(vec![vec!["a", "a", "b"], vec!["aa", "b"]], ret);
    }
}
