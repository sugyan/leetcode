use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut vd = VecDeque::new();
        vd.push_back(s.clone());
        for (i, c) in s.chars().enumerate() {
            if c.is_ascii_alphabetic() {
                (0..vd.len()).for_each(|_| {
                    if let Some(front) = vd.pop_front() {
                        vd.push_back(
                            front[0..i].to_string()
                                + &c.to_ascii_uppercase().to_string()
                                + &front[i + 1..],
                        );
                        vd.push_back(
                            front[0..i].to_string()
                                + &c.to_ascii_lowercase().to_string()
                                + &front[i + 1..],
                        );
                    }
                });
            }
        }
        vd.into_iter().collect()
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
