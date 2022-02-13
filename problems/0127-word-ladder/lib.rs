use std::collections::{HashMap, VecDeque};

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
        let mut vd = vec![&begin_word].into_iter().collect::<VecDeque<_>>();
        let mut seen = vec![false; word_list.len()];
        let mut answer = 1;
        while !vd.is_empty() {
            for _ in 0..vd.len() {
                if let Some(word) = vd.pop_front() {
                    if *word == end_word {
                        return answer;
                    }
                    for i in 0..word.len() {
                        if let Some(v) = hm.get(&(&word[0..i], &word[i + 1..])) {
                            for &j in v {
                                if !seen[j] {
                                    seen[j] = true;
                                    vd.push_back(&word_list[j]);
                                }
                            }
                        }
                    }
                }
            }
            answer += 1;
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
                vec!["hot", "dot", "dog", "lot", "log", "cog"]
                    .into_iter()
                    .map(String::from)
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
                vec!["hot", "dot", "dog", "lot", "log"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }
}
