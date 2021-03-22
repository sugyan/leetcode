use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut exactly_matches = HashSet::new();
        let mut capitalization = HashMap::new();
        let mut vowel = HashMap::new();
        let to_any_vowels = |s: &String| -> String {
            s.to_ascii_lowercase().replace(
                |c| match c {
                    'a' | 'e' | 'i' | 'o' | 'u' => true,
                    _ => false,
                },
                "*",
            )
        };
        for word in wordlist.iter() {
            exactly_matches.insert(word);
            capitalization
                .entry(word.to_ascii_lowercase())
                .or_insert(word);
            vowel.entry(to_any_vowels(word)).or_insert(word);
        }
        let convert = |s: &String| -> String {
            if exactly_matches.contains(s) {
                return s.clone();
            }
            if let Some(&collect) = capitalization.get(&s.to_ascii_lowercase()) {
                return collect.clone();
            }
            if let Some(&collect) = vowel.get(&to_any_vowels(s)) {
                return collect.clone();
            }
            String::new()
        };
        queries.iter().map(convert).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec!["kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe"],
            Solution::spellchecker(
                vec!["KiTe", "kite", "hare", "Hare"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                vec![
                    "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto"
                ]
                .into_iter()
                .map(str::to_string)
                .collect()
            )
        );
    }
}
