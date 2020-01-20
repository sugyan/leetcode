pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut v: Vec<bool> = vec![false; s.len() + 1];
        v[0] = true;
        for i in 0..v.len() {
            if !v[i] {
                continue;
            }
            for word in word_dict.iter() {
                if let Some(head) = s.get(i..i + word.len()) {
                    if head == *word {
                        v[i + word.len()] = true;
                    }
                }
            }
        }
        v[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::word_break(
                "leetcode".to_string(),
                vec!["leet".to_string(), "code".to_string()]
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            true,
            Solution::word_break(
                "applepenapple".to_string(),
                vec!["apple".to_string(), "pen".to_string()]
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::word_break(
                "catsandog".to_string(),
                vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string()
                ]
            )
        );
    }
}
