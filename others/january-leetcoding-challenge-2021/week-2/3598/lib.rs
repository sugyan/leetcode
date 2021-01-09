use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut hm = HashMap::new();
        for (i, word) in word_list.iter().enumerate() {
            for j in 0..word.len() {
                hm.entry((&word[0..j], &word[j + 1..]))
                    .or_insert_with(Vec::new)
                    .push(i);
            }
        }
        let mut hs = HashSet::new();
        let mut vd = VecDeque::new();
        vd.push_back((&begin_word, 1));
        while let Some((word, i)) = vd.pop_front() {
            if *word == end_word {
                return i;
            }
            for j in 0..word.len() {
                if let Some(v) = hm.get(&(&word[0..j], &word[j + 1..])) {
                    for &k in v.iter() {
                        if !hs.contains(&k) {
                            hs.insert(k);
                            vd.push_back((&word_list[k], i + 1));
                        }
                    }
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            5,
            Solution::ladder_length(
                String::from("hit"),
                String::from("cog"),
                ["hot", "dot", "dog", "lot", "log", "cog"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            0,
            Solution::ladder_length(
                String::from("hit"),
                String::from("cog"),
                ["hot", "dot", "dog", "lot", "log"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
    }
}
