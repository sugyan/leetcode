pub struct Solution;

impl Solution {
    pub fn find_longest_word(s: String, d: Vec<String>) -> String {
        let mut d = d;
        d.sort_unstable_by(|a, b| match b.len().cmp(&a.len()) {
            std::cmp::Ordering::Equal => a.cmp(&b),
            o => o,
        });
        let s = s.as_bytes();
        let can_be_formed = |word: &String| -> bool {
            let mut i = 0;
            for &b in word.as_bytes() {
                while i < s.len() && s[i] != b {
                    i += 1;
                }
                if i == s.len() {
                    return false;
                }
                i += 1;
            }
            true
        };
        d.into_iter().find(can_be_formed).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "apple",
            Solution::find_longest_word(
                String::from("abpcplea"),
                vec![
                    String::from("ale"),
                    String::from("apple"),
                    String::from("monkey"),
                    String::from("plea")
                ]
            )
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "a",
            Solution::find_longest_word(
                String::from("abpcplea"),
                vec![String::from("a"), String::from("b"), String::from("c")]
            )
        )
    }
}
