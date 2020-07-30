use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut hm: HashMap<String, Vec<String>> = HashMap::new();
        Solution::helper(&s, &word_dict, &mut hm)
    }
    fn helper(s: &str, words: &[String], hm: &mut HashMap<String, Vec<String>>) -> Vec<String> {
        if let Some(v) = hm.get(&s.to_string()) {
            return v.clone();
        }
        let mut ret = Vec::new();
        for word in words.iter() {
            if s.starts_with(word) {
                for e in Solution::helper(&s[word.len()..], words, hm).iter() {
                    let mut ss = word.clone();
                    ss.push(' ');
                    ret.push(ss + e);
                }
                if word == s {
                    ret.push(word.clone());
                }
            }
        }
        hm.insert(s.to_string(), ret.clone());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::word_break(
            "catsanddog".to_string(),
            ["cat", "cats", "and", "sand", "dog"]
                .iter()
                .map(|&s| s.to_string())
                .collect(),
        );
        ret.sort();
        assert_eq!(vec!["cat sand dog", "cats and dog"], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::word_break(
            "pineapplepenapple".to_string(),
            ["apple", "pen", "applepen", "pine", "pineapple"]
                .iter()
                .map(|&s| s.to_string())
                .collect(),
        );
        ret.sort();
        assert_eq!(
            vec![
                "pine apple pen apple",
                "pine applepen apple",
                "pineapple pen apple"
            ],
            ret
        );
    }

    #[test]
    fn example_3() {
        let v: Vec<String> = Vec::new();
        assert_eq!(
            v,
            Solution::word_break(
                "catsandog".to_string(),
                ["cats", "dog", "sand", "and", "cat"]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect()
            )
        );
    }
}
