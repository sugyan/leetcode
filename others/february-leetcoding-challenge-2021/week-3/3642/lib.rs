pub struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut chars = s.chars().collect::<Vec<_>>();
        let mut answer = Vec::new();
        Self::dfs(&mut chars, &mut answer, 0);
        answer
    }
    fn dfs(chars: &mut [char], answer: &mut Vec<String>, i: usize) {
        if i == chars.len() {
            answer.push(chars.iter().collect());
        } else {
            Self::dfs(chars, answer, i + 1);
            if chars[i].is_alphabetic() {
                chars[i] = ((chars[i] as u8) ^ (1 << 5)) as char;
                Self::dfs(chars, answer, i + 1);
                chars[i] = ((chars[i] as u8) ^ (1 << 5)) as char;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::letter_case_permutation(String::from("a1b2"));
        ret.sort_unstable();
        assert_eq!(vec!["A1B2", "A1b2", "a1B2", "a1b2"], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::letter_case_permutation(String::from("3z4"));
        ret.sort_unstable();
        assert_eq!(vec!["3Z4", "3z4"], ret);
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec!["12345"],
            Solution::letter_case_permutation(String::from("12345"))
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            vec!["0"],
            Solution::letter_case_permutation(String::from("0"))
        );
    }
}
